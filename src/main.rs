mod data;
mod math;
mod pair;
mod exchange;

use tokio::{sync::mpsc, time};
use crate::{exchange::Tick, pair::TradingPair, math::calculate_z_score};

// 5 - dev, 2880 - prod
const CAPACITY: usize = 5;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Tick>(512);

    tokio::spawn(exchange::spawn_market_stream(tx));

    let mut pair = TradingPair::new(CAPACITY, 360.0, 2.0, 0.5);
    
    let mut vec_x = Vec::with_capacity(CAPACITY);
    let mut vec_y = Vec::with_capacity(CAPACITY);
    let mut spread_vec = Vec::with_capacity(CAPACITY);
    let mut delta_buf = Vec::with_capacity(CAPACITY);

    let mut latest_x: Option<f64> = None;
    let mut latest_y: Option<f64> = None;

    let mut ticker = time::interval(time::Duration::from_secs(60));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            Some(tick) = rx.recv() => {
                match tick.symbol.as_str() {
                    "SOLUSDT" => latest_x = Some(tick.price),
                    "ETHUSDT" => latest_y = Some(tick.price),
                    _ => {}
                }
            }

            _ = ticker.tick() => {
                let (Some(x), Some(y)) = (latest_x, latest_y) else {
                    eprintln!("[metronome] skipping missing price data (sol={:?} eth={:?})", latest_x, latest_y);
                    continue;
                };

                pair.add_prices(x, y, &mut vec_x, &mut vec_y, &mut spread_vec, &mut delta_buf);

                if spread_vec.is_empty() {
                    eprintln!("Accumalating ({}/{})", pair.window_x.len(), CAPACITY);
                    continue;
                }

                let z = calculate_z_score(&spread_vec);
                let signal = pair.generate_signal(z);

                println!(
                    "[engine] State: {:?} | Beta: {:.4} | Half Life: {:.2}min | Z: {:+.4} | Signal: {:?}",
                    pair.state, pair.current_beta, pair.current_half_life, z, signal
                );

                pair.process_signal(signal, x,y);
            }
        }
    }
}