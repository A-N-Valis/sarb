use std::{
    fs::{self, OpenOptions}, 
    io::{BufRead, BufReader, BufWriter, Write}, 
    time::{SystemTime, UNIX_EPOCH}
};

use crate::pair::Position;

const HISTORY_FILE: &str = "history.csv";
const POSITION_FILE: &str = "position.json";

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn save_epoch(pric_x: f64, price_y: f64) {
    let ts = current_timestamp();

    match OpenOptions::new().create(true).append(true).open(HISTORY_FILE) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            if let Err(e) = writeln!(writer, "{},{},{}", ts, pric_x, price_y) {
                eprintln!("[storage] failed to write epoch: {}", e);
            }
        }

        Err(e) => eprintln!("[storage] failed to open history.csv: {}", e)
    }
}

pub fn load_history(capacity: usize) -> (Vec<f64>, Vec<f64>) {
    let file = match fs::File::open(HISTORY_FILE) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("[storage] no history.csv found - cold boot");
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

    println!("[storage] loaded {} epochs from history.csv", prices_x.len());
    (prices_x, prices_y)
}

pub fn save_position(pos: &Option<Position>) {
    match serde_json::to_string(pos) {
        Ok(json) => {
            if let Err(e) = fs::write(POSITION_FILE, json) {
                eprintln!("[storage] failed to write to position.json: {}", e);
            }
        }

        Err(e) => eprintln!("[storage] failed to serialize position: {}", e)
    }
}

pub fn load_position() -> Option<Position> {
    let json = match fs::read_to_string(POSITION_FILE) {
        Ok(s) => s,
        Err(_) => return None,
    };

    match serde_json::from_str::<Option<Position>>(&json) {
        Ok(pos) => {
            if pos.is_some() {
                println!("[storage] active position restored from position.json");
            }

            pos
        }

        Err(e) => {
            eprintln!("[storage] failed to parse position.json: {}", e);
            None
        }
    }
}