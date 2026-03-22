mod data;
mod math;
mod pair;
mod exchange;
mod storage;
mod universe;

use std::collections::HashMap;

use tokio::{sync::mpsc, time};
use crate::{
    exchange::Tick, 
    math::calculate_z_score, 
    pair::Position, 
    storage::{init_storage, load_positions}, 
    universe::{build_symbol_list, build_universe}
};

// 5 - dev, 2880 - prod
const CAPACITY: usize = 5;

#[tokio::main]
async fn main() {
    init_storage();

    let active_positions = load_positions();

    let (tx, mut rx) = mpsc::channel::<Tick>(512);

    tokio::spawn(exchange::spawn_market_stream(build_symbol_list(),tx));

    let mut uni = build_universe(CAPACITY);

    for (pair_name, pair) in uni.iter_mut() {
        let (hist_x, hist_y) = storage::load_history(pair_name, CAPACITY);

        let mut vec_x = Vec::with_capacity(CAPACITY);
        let mut vec_y = Vec::with_capacity(CAPACITY);
        let mut spread_vec = Vec::with_capacity(CAPACITY);
        let mut delta_buf = Vec::with_capacity(CAPACITY);

        for (x, y) in hist_x.into_iter().zip(hist_y.into_iter()) {
            pair.add_prices(x, y, &mut vec_x, &mut vec_y, &mut spread_vec, &mut delta_buf);
        }

        if let Some(pos) = active_positions.get(pair_name) {
            pair.active_position = Some( Position {
                is_short_spread: pos.is_short_spread,
                entry_price_x: pos.entry_price_x,
                entry_price_y: pos.entry_price_y,
                entry_beta: pos.entry_beta,
            });
            println!("[engine][{}] position restored from ledger", pair_name);
        }
    }

    let mut live_prices: HashMap<String, f64> = HashMap::new();
    
    let mut vec_x = Vec::with_capacity(CAPACITY);
    let mut vec_y = Vec::with_capacity(CAPACITY);
    let mut spread_vec = Vec::with_capacity(CAPACITY);
    let mut delta_buf = Vec::with_capacity(CAPACITY);

    let mut ticker = time::interval(time::Duration::from_secs(60));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            Some(tick) = rx.recv() => {
                live_prices.insert(tick.symbol, tick.price);
            }

            _ = ticker.tick() => {
                let pair_keys: Vec<String> = uni.keys().cloned().collect();

                for pair_key in &pair_keys {
                    spread_vec.clear();
                    vec_x.clear();
                    vec_y.clear();
                    delta_buf.clear();

                    let (base_x, base_y) = match universe::get_assets(pair_key) {
                        Some(assets) => assets,
                        None => {
                            eprintln!("[engine] unknown pair key: {}", pair_key);
                            continue;
                        }
                    };

                    let sym_x = format!("{}USDT", base_x);
                    let sym_y = format!("{}USDT", base_y);

                    let (&x, &y) = match (live_prices.get(&sym_x), live_prices.get(&sym_y)) {
                        (Some(x), Some(y)) => (x, y),
                        _ => continue
                    };

                    let pair = match uni.get_mut(pair_key) {
                        Some(p) => p,
                        None => continue
                    };

                    storage::save_epoch(pair_key, x, y);

                    pair.add_prices(x, y, &mut vec_x, &mut vec_y, &mut spread_vec, &mut delta_buf);

                    if spread_vec.is_empty() {
                        eprintln!(
                            "[{}] Accumulating ({}/{})",
                            pair_key, pair.window_x.len(), CAPACITY
                        );
                        continue;
                    }

                    let z = calculate_z_score(&spread_vec);
                    let signal = pair.generate_signal(z);

                    println!(
                        "[{}] State: {:?} | Beta: {:.3} | Half-Life: {:.3}min | Z: {:+.4} | Signal: {:?}",
                        pair_key, pair.state, pair.current_beta,
                        pair.current_half_life, z, signal
                    );

                    let had_position = pair.active_position.is_some();
                    pair.process_signal(signal, x, y);
                    let has_position = pair.active_position.is_some();

                    if had_position != has_position {
                        storage::save_positions(&uni);
                    }
                }
            }
        }
    }
}