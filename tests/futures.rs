#[allow(dead_code)]
mod common;

use ftx_rest_client::{
    endpoints::futures::{
        GetExpiredFutures, GetFundingRates, GetFuture, GetFutureStats, GetFutures,
    },
    Response,
};

#[tokio::test]
#[ignore]
async fn get_futures() {
    common::make_request(&GetFutures)
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_future() {
    common::make_request(&GetFuture { future: "BTC-PERP" })
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_future_stats() {
    common::make_request(&GetFutureStats { future: "BTC-PERP" })
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_funding_rates() {
    common::make_request(&GetFundingRates {
        perpetual: Some("BTC-PERP"),
        start_time: None,
        end_time: None,
    })
    .await
    .deserialize()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_expired_futures() {
    common::make_request(&GetExpiredFutures)
        .await
        .deserialize()
        .unwrap();
}
