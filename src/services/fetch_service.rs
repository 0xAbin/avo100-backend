

pub static totexc  : u8 = 5 ;

pub static bybit : &str = "wss://stream.bybit.com/realtime_public";
pub static coinbase : &str = "wss://ws-feed.exchange.coinbase.com";
pub static okk : &str = "wss://ws.okx.com:8443/ws/v5/public";
pub static bitget : &str = "wss://wsapi.bitget.com/mix/v1/stream";


pub static token : &str = "BTC";

pub fn binance_get(symbol: &str) -> String {
    let symbol_lower = symbol.to_lowercase();
    format!("wss://stream.binance.com:9443/ws/{}usdt@trade", symbol_lower)
}

pub fn bybit_get(symbol: &str) -> String {
    let symbol_upper = symbol.to_uppercase();  
    format!(r#"{{"op": "subscribe", "args": ["trade.{}"]}}"#, symbol_upper)
}


pub fn coinbase_get(symbol: &str) -> String {
    let symbol_upper = symbol.to_uppercase();  
    format!(r#"{{"type": "subscribe", "channels": [{{"name": "ticker", "product_ids": ["{}-USD"]}}]}}"#, symbol_upper)
}


pub fn okx_get(symbol: &str) -> String {
    let symbol_upper = symbol.to_uppercase();
    format!(r#"{{"op": "subscribe", "args": [{{"channel": "trades", "instId": "{}-USDT"}}]}}"#, symbol_upper)
}


pub fn bitget_get(symbol: &str) -> String {
    let symbol_upper = symbol.to_uppercase(); 
    format!(r#"{{"op": "subscribe", "args": ["spot/trade:{}-USDT"]}}"#, symbol_upper)
}


