


// make sure the struct is public default is private 
pub struct Price {
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
}


pub fn get_assets_price() -> Price {

    let symbol  =  String::from("BTC");
    let price = 10000.0;
    let timestamp = 1620000000;

    Price {
        symbol,
        price,
        timestamp,
    }
}

