use std::collections::{HashMap, HashSet};
use crate::pair::TradingPair;

const PAIR_DEFINITIONS: &[(&str, &str, &str)] = &[
    ("SOL-AVAX",    "SOL",   "AVAX"),
    ("SOL-ADA",     "SOL",   "ADA"),
    ("SOL-SUI",     "SOL",   "SUI"),
    ("AVAX-ADA",    "AVAX",  "ADA"),
    ("AVAX-SUI",    "AVAX",  "SUI"),
    ("ADA-SUI",     "ADA",   "SUI"),
    ("FET-WLD",     "FET",   "WLD"),
    ("FET-TAO",     "FET",   "TAO"),
    ("WLD-TAO",     "WLD",   "TAO"),
    ("LINK-FIL",    "LINK",  "FIL"),
    ("LINK-ETHFI",  "LINK",  "ETHFI"),
    ("LINK-RDNT",   "LINK",  "RDNT"),
    ("FIL-ETHFI",   "FIL",   "ETHFI"),
    ("FIL-RDNT",    "FIL",   "RDNT"),
    ("ETHFI-RDNT",  "ETHFI", "RDNT"),
];

pub fn build_universe(capacity: usize) -> HashMap<String, TradingPair> {
    PAIR_DEFINITIONS
        .iter()
        .map(|&(key, _, _)| (key.to_string(), TradingPair::new(capacity, 360.0, 2.0, 0.5)))
        .collect()
}

pub fn build_symbol_list() -> Vec<String> {
    let mut seen = HashSet::new();
    let mut symbols = Vec::new();

    for &(_, x, y) in PAIR_DEFINITIONS {
        if seen.insert(x) {
            symbols.push(format!("{}USDT", x));
        }

        if seen.insert(y) {
            symbols.push(format!("{}USDT", y));
        }
    }

    symbols
}

pub fn get_assets(pair_key: &str) -> Option<(&'static str, &'static str)> {
    PAIR_DEFINITIONS
        .iter()
        .find(|&&(key, _, _)| key == pair_key)
        .map(|&(_, x, y)| (x, y))
}