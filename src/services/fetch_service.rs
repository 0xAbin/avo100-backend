

pub static totexc  : &u8 = 5;

pub static binance : &str = "wss://stream.binance.com:9443/ws";
pub static bybit : &str = "wss://stream.bybit.com/realtime_public";
pub static coinbase : &str = "wss://ws-feed.exchange.coinbase.com";
pub static okk : &str = "wss://ws.okx.com:8443/ws/v5/public";
pub static bitget : &str = "wss://wsapi.bitget.com/mix/v1/stream";


//  for bybit
// {
//     "op": "subscribe",
//     "args": ["trade.BTCUSD"]
//   }
//

//  for coinbase
// {
//     "type": "subscribe",
//     "channels": [{"name": "ticker", "product_ids": ["BTC-USD"]}]
// }
//

//  for okk
// {
//     "op": "subscribe",
//     "args": [
//       {"channel": "trades", "instId": "BTC-USDT"}
//     ]
//   }
//

//  for bitget
//   {
//       "op": "subscribe",
//       "args": ["spot/trade:BTC-USDT"]
//   }
//