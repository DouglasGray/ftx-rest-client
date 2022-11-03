#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{endpoints::funding_payments::GetFundingPayments, Response};

#[tokio::test]
#[ignore]
async fn get_funding_payments() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetFundingPayments {
            future: None,
            start_time: None,
            end_time: None,
        },
    )
    .await
    .deserialize()
    .unwrap();
}
