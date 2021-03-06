#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    endpoints::spot_margin::{
        GetBorrowForMarket, GetBorrowHistory, GetBorrowRates, GetDailyBorrowedAmounts,
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
        .to_data()
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
        .to_data()
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
    .to_data()
    .unwrap();

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetBorrowForMarket {
            spot_market: "BTC/USD",
        },
    )
    .await
    .to_data()
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
    .to_data()
    .unwrap();
}
