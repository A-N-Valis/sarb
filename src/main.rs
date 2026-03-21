mod data;
mod math;
mod pair;

use crate::{
    math::calculate_z_score,
    pair::TradingPair
};

fn main() {
    let mut trading_pair = TradingPair::new(10, 100.0, 2.0, 0.5);

    let mut vec_x = Vec::with_capacity(10);
    let mut vec_y = Vec::with_capacity(10);
    let mut spread_vec = Vec::with_capacity(10);
    let mut delta_buf = Vec::with_capacity(10);

    for i in 1..=20 {
        let price_x = 100.0 + i as f64 * 0.5;
        let price_y = if i == 18 { 240.0 } else { 200.0 + i as f64 * 1.1 };

        trading_pair.add_prices(price_x, price_y, &mut vec_x, &mut vec_y, &mut spread_vec, &mut delta_buf);

        if !spread_vec.is_empty() {
            let z = calculate_z_score(&spread_vec);
            let signal = trading_pair.generate_signal(z);

            println!(
                "i={} | Z: {:.2} | State: {:?} | Signal: {:?}",
                i, z, trading_pair.state, signal
            );

            trading_pair.process_signal(signal, price_x, price_y);
        }
    }
}