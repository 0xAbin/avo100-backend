use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use std::collections::HashSet;

pub fn get_assets() -> Result<Vec<(usize, String)>, Box<dyn std::error::Error>> {

    dotenv().ok();
    let url = env::var("URL").expect("URL must be set");


    let response = get(&url)?.text()?;
    let data: Value = serde_json::from_str(&response)?;

    // Need to work on this 
    let fields = data["fields"].as_array().expect("Expected fields array");
    let rank_index = fields
        .iter()
        .position(|field| field.as_str() == Some("rank"))
        .expect("Rank field not found");
    let symbol_index = fields
        .iter()
        .position(|field| field.as_str() == Some("symbol"))
        .expect("Symbol field not found");

    let values = data["values"].as_array().expect("Expected values array");
    let excluded_symbols: HashSet<&str> = ["USDT", "USDC"].iter().cloned().collect();

    let mut rank_symbol_pairs: Vec<(i64, String)> = values
        .iter()
        .filter_map(|entry| {
            let rank = entry[rank_index].as_i64()?;
            let symbol = entry[symbol_index].as_str()?;
            if rank > 0 && !excluded_symbols.contains(symbol) {
                Some((rank, symbol.to_string()))
            } else {
                None
            }
        })
        .collect();

    rank_symbol_pairs.sort_by_key(|k| k.0);

    let top_100: Vec<(usize, String)> = rank_symbol_pairs
        .into_iter()
        .take(100)
        .enumerate()
        .map(|(i, (_, symbol))| (i + 1, symbol)) 
        .collect();

    Ok(top_100)
}