use reqwest::blocking::get;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Asset {
    pub symbol: String,
}

#[derive(Deserialize)]
struct Resonse {
    values : Vec<Vec<serde_json::Value>>,
}

// pub fn get_assets() -> Result<Vec<Asset>, Error> {

//     dotenv().ok();
    
//     let url = env::var("URL").expect("URL must be set");

//     println!("URL: {}", url);
  
// }

pub fn get_assets() {
    
    dotenv().ok();

    let url = env::var("URL").expect("URL must be set");

    println!("URL: {}", url);
}