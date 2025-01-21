mod utils;
use utils::get_assets::get_assets;

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
}