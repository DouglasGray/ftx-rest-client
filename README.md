### FTX REST API client

An FTX REST API client written in Rust, docs
[here](https://docs.ftx.com/#rest-api).

Can be used by first creating a client (requires an authenticated
client for authenticated endpoints), and then passing it an instance
of the desired request. For example:

```rust
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

    println!("{:#?}", response.deserialize().unwrap());
}
```
