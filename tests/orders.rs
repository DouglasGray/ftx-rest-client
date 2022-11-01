#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    data::Side,
    endpoints::orders::{
        CancelAllOrders, CancelOrder, EditOrder, EditOrderOpts, GetOpenOrders, GetOrderStatus,
        OrderId, PlaceOrder,
    },
    Response,
};
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
#[ignore]
async fn place_limit_order_then_check_status() {
    if !CONFIG.perform_auth_api_tests || !CONFIG.perform_order_placement_tests {
        return;
    }

    let market = "ETH-PERP";
    let client = AuthTestClient::new_for_subaccount();

    let order_id = common::make_auth_request(
        &client,
        &PlaceOrder {
            market: &market,
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.001".parse().unwrap(),
            client_id: None,
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap()
    .id;

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.id == order_id);

    assert!(has_id);

    common::make_auth_request(
        &client,
        &GetOrderStatus {
            order_id: OrderId::Exchange(order_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    common::make_auth_request(
        &client,
        &CancelOrder {
            order_id: OrderId::Exchange(order_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.id == order_id);

    assert!(!has_id);
}

#[tokio::test]
#[serial]
#[ignore]
async fn place_limit_order_then_check_status_with_client_id() {
    if !CONFIG.perform_auth_api_tests || !CONFIG.perform_order_placement_tests {
        return;
    }

    let market = "ETH-PERP";
    let client = AuthTestClient::new_for_subaccount();

    let client_id = Uuid::new_v4().to_string();

    common::make_auth_request(
        &client,
        &PlaceOrder {
            market: &market,
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.001".parse().unwrap(),
            client_id: Some(&client_id),
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap()
    .id;

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.client_id == Some(&client_id));

    assert!(has_id);

    common::make_auth_request(
        &client,
        &GetOrderStatus {
            order_id: OrderId::Client(&client_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    common::make_auth_request(
        &client,
        &CancelOrder {
            order_id: OrderId::Client(&client_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.client_id == Some(&client_id));

    assert!(!has_id);
}

#[tokio::test]
#[serial]
#[ignore]
async fn place_limit_order_then_edit() {
    if !CONFIG.perform_auth_api_tests || !CONFIG.perform_order_placement_tests {
        return;
    }

    let market = "ETH-PERP";
    let client = AuthTestClient::new_for_subaccount();

    let new_price = "2".parse().unwrap();
    let new_size = "0.002".parse().unwrap();

    let order_id = common::make_auth_request(
        &client,
        &PlaceOrder {
            market: &market,
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.001".parse().unwrap(),
            client_id: None,
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap()
    .id;

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.id == order_id);

    assert!(has_id);

    let order_edit_response = common::make_auth_request(
        &client,
        &EditOrder {
            order_id: OrderId::Exchange(order_id),
            opts: EditOrderOpts {
                price: Some(new_price),
                size: Some(new_size),
                client_id: None,
            },
        },
    )
    .await;

    let order_edit = order_edit_response.deserialize().unwrap();

    assert_eq!(order_edit.price, new_price);
    assert_eq!(order_edit.size, new_size);

    let new_order_id = order_edit.id;

    common::make_auth_request(
        &client,
        &CancelOrder {
            order_id: OrderId::Exchange(new_order_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.id == new_order_id);

    assert!(!has_id);
}

#[tokio::test]
#[serial]
#[ignore]
async fn place_limit_order_then_edit_with_client_id() {
    if !CONFIG.perform_auth_api_tests || !CONFIG.perform_order_placement_tests {
        return;
    }

    let market = "ETH-PERP";
    let client = AuthTestClient::new_for_subaccount();

    let client_id = Uuid::new_v4().to_string();

    let new_price = "2".parse().unwrap();
    let new_size = "0.002".parse().unwrap();
    let new_client_id = Uuid::new_v4().to_string();

    common::make_auth_request(
        &client,
        &PlaceOrder {
            market: &market,
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.001".parse().unwrap(),
            client_id: Some(&client_id),
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap();

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.client_id == Some(&client_id));

    assert!(has_id);

    let order_edit_response = common::make_auth_request(
        &client,
        &EditOrder {
            order_id: OrderId::Client(&client_id),
            opts: EditOrderOpts {
                price: Some(new_price),
                size: Some(new_size),
                client_id: Some(&new_client_id),
            },
        },
    )
    .await;

    let order_edit = order_edit_response.deserialize().unwrap();

    assert_eq!(order_edit.price, new_price);
    assert_eq!(order_edit.size, new_size);
    assert_eq!(order_edit.client_id, Some(new_client_id.as_str()));

    common::make_auth_request(
        &client,
        &CancelOrder {
            order_id: OrderId::Client(&new_client_id),
        },
    )
    .await
    .deserialize()
    .unwrap();

    let has_id = common::make_auth_request(
        &client,
        &GetOpenOrders {
            market: Some(&market),
        },
    )
    .await
    .deserialize()
    .unwrap()
    .into_iter()
    .any(|o| o.client_id == Some(&new_client_id));

    assert!(!has_id);
}

#[tokio::test]
#[serial]
#[ignore]
async fn cancel_all_orders() {
    if !CONFIG.perform_auth_api_tests || !CONFIG.perform_order_placement_tests {
        return;
    }

    let client = AuthTestClient::new_for_subaccount();

    let btc_order_id = common::make_auth_request(
        &client,
        &PlaceOrder {
            market: "SOL-PERP",
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.01".parse().unwrap(),
            client_id: None,
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap()
    .id;

    let eth_order_id = common::make_auth_request(
        &client,
        &PlaceOrder {
            market: "ETH-PERP",
            price: Some("1".parse().unwrap()),
            side: Side::Buy,
            size: "0.001".parse().unwrap(),
            client_id: None,
            opts: None,
        },
    )
    .await
    .deserialize()
    .unwrap()
    .id;

    let open_orders_resp =
        common::make_auth_request(&client, &GetOpenOrders { market: None }).await;

    let open_orders = open_orders_resp.deserialize().unwrap();

    let has_btc_order_id = open_orders.iter().any(|o| o.id == btc_order_id);

    let has_eth_order_id = open_orders.iter().any(|o| o.id == eth_order_id);

    assert!(has_btc_order_id);
    assert!(has_eth_order_id);

    common::make_auth_request(
        &client,
        &CancelAllOrders {
            market: None,
            side: None,
            limit_orders_only: None,
        },
    )
    .await
    .deserialize()
    .unwrap();

    let open_orders_resp =
        common::make_auth_request(&client, &GetOpenOrders { market: None }).await;

    let open_orders = open_orders_resp.deserialize().unwrap();

    let has_btc_order_id = open_orders.iter().any(|o| o.id == btc_order_id);

    let has_eth_order_id = open_orders.iter().any(|o| o.id == eth_order_id);

    assert!(!has_btc_order_id);
    assert!(!has_eth_order_id);
}
