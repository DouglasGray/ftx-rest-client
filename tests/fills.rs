#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{endpoints::fills::GetFills, Response};

#[tokio::test]
#[ignore]
async fn get_fills() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    common::make_auth_request(
        &AuthTestClient::new_for_main(),
        &GetFills {
            market: None,
            start_time: None,
            end_time: None,
            order: None,
            order_id: None,
        },
    )
    .await
    .deserialize_partial()
    .unwrap();
}
