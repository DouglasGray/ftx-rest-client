#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    endpoints::spot_margin::{
        GetBorrowForMarket, GetBorrowHistory, GetBorrowRates, GetDailyBorrowedAmounts,
        GetLendingRates,
    },
    Response,
};

#[tokio::test]
#[ignore]
async fn get_borrow_rates() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(&AuthTestClient::new_for_main(), &GetBorrowRates)
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_lending_rates() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_request(&GetLendingRates)
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_daily_borrowed_amounts() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(&AuthTestClient::new_for_main(), &GetDailyBorrowedAmounts)
        .await
        .deserialize()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_borrow_for_market() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    // Will be null but should parse
    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetBorrowForMarket {
            spot_market: "BTC-PERP",
        },
    )
    .await
    .deserialize()
    .unwrap();

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetBorrowForMarket {
            spot_market: "BTC/USD",
        },
    )
    .await
    .deserialize()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_borrow_history() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetBorrowHistory {
            start_time: None,
            end_time: None,
        },
    )
    .await
    .deserialize()
    .unwrap();
}
