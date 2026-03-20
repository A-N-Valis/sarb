use crate::data::PriceWindow;

mod data;

fn main() {
    let mut window = PriceWindow::new(5);

    for i in 1..=7 {
        window.push(i as f64);
    }

    println!("is_ready: {}", window.is_ready());
    println!("to_vec: {:?}", window.to_vec());
}
