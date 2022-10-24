use ftx_rest_client::{endpoints::markets::GetMarket, Client, Executor, Response};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let response = client
        .execute(
            &GetMarket {
                market: &"BTC-PERP",
            },
            Some(Duration::from_secs(10)),
        )
        .await
        .unwrap();

    println!("{:#?}", response.parse().unwrap());
}
