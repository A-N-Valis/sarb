mod data;
mod math;
mod pair;

use crate::pair::TradingPair;

fn main() {
    let mut trading_pair = TradingPair::new(10, 100.0);

    let mut vec_x = Vec::with_capacity(10);
    let mut vec_y = Vec::with_capacity(10);
    let mut spread_vec = Vec::with_capacity(10);
    let mut delta_buf = Vec::with_capacity(10);

    for i in 1..=15 {
        let price_x = 100.0 + i as f64 * 0.5;
        let price_y = 200.0 + i as f64 * 1.1;
        trading_pair.add_prices(price_x, price_y, &mut vec_x, &mut vec_y, &mut spread_vec, &mut delta_buf);
    }

    println!("PairState: {:?}", trading_pair.state);
    println!("Beta: {:.4}", trading_pair.current_beta);
    println!("Half Life: {:.4}", trading_pair.current_half_life);
}
