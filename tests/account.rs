#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    endpoints::account::{
        AccountLeverage, ChangeAccountLeverage, GetAccountInformation, GetPositions,
    },
    Response,
};

#[tokio::test]
#[ignore]
async fn get_account_information() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_subaccount();

    common::make_auth_request(&client, &GetAccountInformation)
        .await
        .parse()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_positions() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_subaccount();

    common::make_auth_request(
        &client,
        &GetPositions {
            show_avg_price: Some(true),
        },
    )
    .await
    .parse()
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn change_account_leverage_then_change_it_back() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_subaccount();

    let old_leverage = common::make_auth_request(&client, &GetAccountInformation)
        .await
        .parse()
        .unwrap()
        .leverage;

    common::make_auth_request(
        &client,
        &ChangeAccountLeverage {
            leverage: AccountLeverage::Twenty,
        },
    )
    .await
    .parse()
    .unwrap();

    common::make_auth_request(
        &client,
        &ChangeAccountLeverage {
            leverage: old_leverage,
        },
    )
    .await
    .parse()
    .unwrap();
}
