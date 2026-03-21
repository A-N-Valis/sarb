mod data;
mod math;
mod pair;
mod exchange;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(exchange::spawn_market_stream(tx));

    while let Some(update) = rx.recv().await {
        println!("[{}]  price: {:.3}", update.symbol, update.price);
    }
}