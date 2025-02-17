mod utils;
use utils::get_assets::get_assets;

mod services;
use services::fetch_service::{binance_get, bybit_get, coinbase_get, okx_get, bitget_get};

use services::streaming_service::get_assets_price;


fn main() {
    // match get_assets() {
    //     Ok(rank_symbol_pairs) => {
    //         for (rank, symbol) in rank_symbol_pairs {

    //             let binance_url = binance_get(&symbol);
    //             let bybit_url = bybit_get(&symbol);
    //             let coinbase_url = coinbase_get(&symbol);
    //             let okx_url = okx_get(&symbol);
    //             let bitget_url = bitget_get(&symbol);

    //             println!("---------------------------------");
    //             println!("{}", rank);
    //             println!("---------------------------------");
    //             println!("{}", binance_url);
             
    //             println!("{}", bybit_url);
               
    //             println!("{}", coinbase_url);
              
    //             println!("{}", okx_url);

    //             println!("{}", bitget_url);

    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("Error: {:?}", err);
    //     }
    // }

    let asset_price = get_assets_price();
    print!("{} ", asset_price.symbol);
    print!("{} ", asset_price.price);
    print!("{} ", asset_price.timestamp);
      
}