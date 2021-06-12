#[allow(dead_code)]
mod common;

use ftx_rest_client::{
    data::WindowLength,
    endpoints::markets::{BookDepth, GetCandles, GetMarket, GetMarkets, GetOrderBook, GetTrades},
    Response,
};
use std::convert::TryInto;

#[tokio::test]
#[ignore]
async fn get_markets() {
    common::make_request(&GetMarkets).await.to_data().unwrap();
}

#[tokio::test]
#[ignore]
async fn get_market() {
    common::make_request(&GetMarket { market: "BTC-PERP" })
        .await
        .to_data()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_orderbook() {
    common::make_request(&GetOrderBook {
        market: "BTC-PERP",
        depth: Some(BookDepth::new(20.try_into().unwrap()).unwrap()),
    })
    .await
    .to_data()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_trades() {
    common::make_request(&GetTrades {
        market: "BTC-PERP",
        start_time: None,
        end_time: None,
    })
    .await
    .to_data()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_candles() {
    common::make_request(&GetCandles {
        market: "BTC-PERP",
        resolution: WindowLength::OneMinute,
        start_time: None,
        end_time: None,
    })
    .await
    .to_data()
    .unwrap();
}
