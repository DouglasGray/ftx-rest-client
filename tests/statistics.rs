#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{endpoints::statistics::GetLatencyStatistics, Response};

#[tokio::test]
#[ignore]
async fn get_latency_statistics() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetLatencyStatistics {
            days: None,
            subaccount_nickname: None,
        },
    )
    .await
    .deserialize()
    .unwrap();
}
