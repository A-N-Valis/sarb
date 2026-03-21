use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde::Deserialize;

#[derive(Debug)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
}

#[derive(Deserialize)]
struct AggTradeData {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "p")]
    price: String,
}

#[derive(Deserialize)]
struct CombinedMsg {
    data: AggTradeData
}

async fn connect_and_stream(url: &str, tx: &mpsc::Sender<Tick>) -> anyhow::Result<()> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                if let Ok(parsed) = serde_json::from_str::<CombinedMsg>(&text) {
                    if let Ok(price) = parsed.data.price.parse::<f64>() {
                        let update = Tick {
                            symbol: parsed.data.symbol,
                            price
                        };

                        if tx.send(update).await.is_err() {
                            return Ok(());
                        }
                    }
                }
            }

            Message::Ping(payload) => {
                write.send(Message::Pong(payload)).await?;
            }

            Message::Close(_) => {
                eprintln!("[exchange] received close frame");
                break;
            }

            _ => {}
        }
    }

    Ok(())
}

pub async fn spawn_market_stream(symbols: Vec<String>, tx: mpsc::Sender<Tick>) {
    let streams = symbols
        .iter()
        .map(|s| format!("{}@aggTrade", s.to_lowercase()))
        .collect::<Vec<String>>()
        .join("/");

    let url = format!("wss://stream.binance.com:9443/stream?streams={}", streams);

    loop {
        match connect_and_stream(&url, &tx).await {
            Ok(_) => eprintln!("[exchange] stream closed reconnecting in 5s"),
            Err(e) => eprintln!("[exchange] Error: {} reconnecting in 5s", e),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}