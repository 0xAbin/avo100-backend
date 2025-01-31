mod utils;
use utils::get_assets::get_assets;

mod services;
use services::fetch_service::binance_get;


fn main() {
    match get_assets() {
        Ok(rank_symbol_pairs) => {
            for (rank, symbol) in rank_symbol_pairs {
                println!("{}: {}", rank, symbol);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
    let symbol = "BTC";

  let binance_url = binance_get(&symbol);
  let bybit_url = services::fetch_service::bybit_get(&symbol);
  let coinbase_url = services::fetch_service::coinbase_get(&symbol);
  let okx_url = services::fetch_service::okx_get(&symbol);
  let bitget_url = services::fetch_service::bitget_get(&symbol);
 
  println!("Binance WebSocket URL: {}", binance_url);
  println!("Bybit WebSocket URL: {}", bybit_url);
  println!("Coinbase WebSocket URL: {}", coinbase_url);
  println!("OKX WebSocket URL: {}", okx_url);
  println!("Bitget WebSocket URL: {}", bitget_url);
      
}