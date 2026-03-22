use std::{
    collections::HashMap, 
    fs::{self, OpenOptions}, 
    io::{BufRead, BufReader, BufWriter, Write}, 
    time::{SystemTime, UNIX_EPOCH}
};

use crate::pair::{Position, TradingPair};

const DATA_DIR: &str = "data";
const POSITIONS_FILE: &str = "data/active_positions.json";

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn history_path(pair_name: &str) -> String {
    format!("{}/history_{}.csv", DATA_DIR, pair_name)
}

pub fn init_storage() {
    if let Err(e) = fs::create_dir_all(DATA_DIR) {
        eprintln!("[storage] failed to create data dir: {}", e);
    }
}

pub fn save_epoch(pair_name: &str, price_x: f64, price_y: f64) {
    let ts = current_timestamp();
    let path = history_path(pair_name);

    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            if let Err(e) = writeln!(writer, "{},{},{}", ts, price_x, price_y) {
                eprintln!("[storage] [{}] failed to write epoch: {}", pair_name, e);
            }
        }

        Err(e) => eprintln!("[storage] [{}] failed to open {}: {}", pair_name, path, e)
    }
}

pub fn load_history(pair_name: &str, capacity: usize) -> (Vec<f64>, Vec<f64>) {
    let path = history_path(pair_name);

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("[storage] [{}] no history found - cold boot", pair_name);
            return (Vec::new(), Vec::new());
        }
    };

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .collect();

    let slice = if lines.len() > capacity {
        &lines[lines.len() - capacity ..]
    } else {
        &lines[..]
    };

    let mut prices_x = Vec::with_capacity(slice.len());
    let mut prices_y = Vec::with_capacity(slice.len());

    for line in slice {
        let mut parts = line.splitn(3, ',');
        let _ts = parts.next();
        let px = parts.next().and_then(|s| s.parse::<f64>().ok());
        let py = parts.next().and_then(|s| s.parse::<f64>().ok());

        if let (Some(x), Some(y)) = (px, py) {
            prices_x.push(x);
            prices_y.push(y);
        }
    }

    println!("[storage] [{}] loaded {} epochs from history.csv", pair_name, prices_x.len());

    (prices_x, prices_y)
}

pub fn save_positions(universe: &HashMap<String, TradingPair>) {
    let live: HashMap<&String, &Position> = universe
        .iter()
        .filter_map(|(name, pair)| pair.active_position.as_ref().map(|pos| (name, pos)))
        .collect();

    let file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(POSITIONS_FILE)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[storage] failed to open active_positions.json: {}", e);
            return;
        }
    };

    let writer = BufWriter::new(file);
    if let Err(e) = serde_json::to_writer_pretty(writer, &live) {
        eprintln!("[storage] failed to serialize positions: {}", e);
    }

}

pub fn load_positions() -> HashMap<String, Position> {
    let json = match fs::read_to_string(POSITIONS_FILE) {
        Ok(s) => s,
        Err(_) => return HashMap::new(),
    };

    match serde_json::from_str::<HashMap<String, Position>>(&json) {
        Ok(map) => {
            println!("[storage] restored {} active positions", map.len());
            map
        }

        Err(e) => {
            eprintln!("[storage] failed to parse position.json: {}", e);
            HashMap::new()
        }
    }
}