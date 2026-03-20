mod data;
mod math;

use crate::{
    data::PriceWindow, 
    math::{calculate_beta, calculate_half_life, calculate_spread}
};

fn main() {
    let mut window = PriceWindow::new(5);
    let mut spread_vec = Vec::new();
    let mut slice_vec = Vec::new();
    let mut delta_buf = Vec::new();

    for i in 1..=7 {
        window.push(i as f64);
    }

    window.fill_slice(&mut slice_vec);

    println!("is_ready: {}", window.is_ready());
    println!("to_vec: {:?}", slice_vec);

    let x = vec![100.0, 101.5, 102.0, 103.5, 104.0];
    let y = vec![200.5, 203.2, 204.1, 207.3, 208.0];

    let beta = calculate_beta(&x, &y);
    calculate_spread(&x, &y, beta, &mut spread_vec);

    println!("Beta: {:.3}", beta);
    println!("Spread: {:?}", spread_vec);

    let half_life = calculate_half_life(&spread_vec, &mut delta_buf);
    println!("Half Life: {}", half_life);
}
