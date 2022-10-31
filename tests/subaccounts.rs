#[allow(dead_code)]
mod common;
use common::{AuthTestClient, CONFIG};

use ftx_rest_client::{
    data::NonNegativeDecimal,
    endpoints::subaccounts::{
        ChangeSubaccountName, CreateSubaccount, DeleteSubaccount, GetSubaccountBalances,
        GetSubaccounts, TransferBetweenSubaccounts,
    },
    Response,
};
use rust_decimal::Decimal;
use std::convert::TryInto;
use tokio::task;

#[tokio::test]
#[ignore]
async fn get_subaccounts() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_main();

    common::make_auth_request(&client, &GetSubaccounts)
        .await
        .deserialize_partial()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn get_subaccount_balances() {
    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_main();

    common::make_auth_request(&client, &GetSubaccountBalances { nickname: "main" })
        .await
        .deserialize_partial()
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn create_subaccount_then_change_its_name_then_delete_it() {
    const NICKNAME: &str = "ftx_client_test1";
    const NEW_NICKNAME: &str = "ftx_client_test2";

    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_main();

    common::make_auth_request(&client, &CreateSubaccount { nickname: NICKNAME })
        .await
        .deserialize_partial()
        .unwrap();

    // Put test in separate task so we can easily clean up the
    // throwaway subaccount regardless of the outcome.
    let res = task::spawn(async move {
        let client = AuthTestClient::new_for_main();

        common::make_auth_request(
            &client,
            &ChangeSubaccountName {
                nickname: NICKNAME,
                new_nickname: NEW_NICKNAME,
            },
        )
        .await
        .deserialize_partial()
        .unwrap();

        common::make_auth_request(
            &client,
            &DeleteSubaccount {
                nickname: NEW_NICKNAME,
            },
        )
        .await
        .deserialize_partial()
        .unwrap();
    })
    .await;

    // If error then try delete the first throwaway account we
    // created.
    if res.is_err() {
        common::make_auth_request(&client, &DeleteSubaccount { nickname: NICKNAME })
            .await
            .deserialize_partial()
            .unwrap();
    }

    res.unwrap();
}

#[tokio::test]
#[ignore]
async fn try_transfer_one_usd_from_main_to_subaccount_then_back() {
    const NICKNAME: &str = "ftx_client_test";

    if !CONFIG.perform_auth_api_tests {
        return;
    }

    let client = AuthTestClient::new_for_main();

    let should_transfer =
        common::make_auth_request(&client, &GetSubaccountBalances { nickname: "main" })
            .await
            .deserialize_partial()
            .unwrap()
            .iter()
            .any(|b| b.coin == "USD" && b.free >= NonNegativeDecimal::from(1u64));

    if should_transfer {
        common::make_auth_request(&client, &CreateSubaccount { nickname: NICKNAME })
            .await
            .deserialize_partial()
            .unwrap();

        // Put test in separate task so we can easily clean up the
        // throwaway subaccount regardless of the outcome.
        let res = task::spawn(async move {
            let client = AuthTestClient::new_for_main();

            common::make_auth_request(
                &client,
                &TransferBetweenSubaccounts {
                    coin: "USD",
                    size: Decimal::ONE.try_into().unwrap(),
                    source: None,
                    destination: Some(NICKNAME),
                },
            )
            .await
            .deserialize_partial()
            .unwrap();

            common::make_auth_request(
                &client,
                &TransferBetweenSubaccounts {
                    coin: "USD",
                    size: Decimal::ONE.try_into().unwrap(),
                    source: Some(NICKNAME),
                    destination: None,
                },
            )
            .await
            .deserialize_partial()
            .unwrap();
        })
        .await;

        common::make_auth_request(&client, &DeleteSubaccount { nickname: NICKNAME })
            .await
            .deserialize_partial()
            .unwrap();

        res.unwrap()
    }
}
