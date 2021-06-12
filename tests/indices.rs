#[allow(dead_code)]
mod common;

use ftx_rest_client::{
    data::WindowLength,
    endpoints::indices::{GetCandles, GetConstituents, GetWeights},
    Response,
};

#[tokio::test]
#[ignore]
async fn get_weights() {
    common::make_request(&GetWeights { index: "SHIT" })
        .await
        .to_data()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_candles() {
    common::make_request(&GetCandles {
        index: "SHIT",
        resolution: WindowLength::OneMinute,
        start_time: None,
        end_time: None,
    })
    .await
    .to_data()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_constituents() {
    common::make_request(&GetConstituents { underlying: "BTC" })
        .await
        .to_data()
        .unwrap();
}
