#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    endpoints::wallet::{GetAllBalances, GetBalances, GetCoins},
    Response,
};

#[tokio::test]
#[ignore]
async fn get_coins() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(&AuthTestClient::new_for_main(), &GetCoins)
        .await
        .deserialize_partial()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_balances() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(&AuthTestClient::new_for_main(), &GetBalances)
        .await
        .deserialize_partial()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_all_balances() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(&AuthTestClient::new_for_main(), &GetAllBalances)
        .await
        .deserialize_partial()
        .unwrap();
}
