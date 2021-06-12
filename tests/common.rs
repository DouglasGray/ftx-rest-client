use async_trait::async_trait;
use bytes::Bytes;
use config::{Config, Environment};
use crossbeam_channel::{Receiver, Sender};
use dotenv::dotenv;
use ftx_rest_client::{
    auth::Authenticator, error::Error, AuthClient, AuthExecutor, Client, Executor, Request,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{thread, time::Duration};

pub static CONFIG: Lazy<TestConfig> = Lazy::new(|| TestConfig::new().unwrap());

static THROTTLER: Lazy<Receiver<Sender<()>>> = Lazy::new(|| {
    let (tx, rx) = crossbeam_channel::bounded(0);

    thread::spawn(move || loop {
        let (completion_tx, completion_rx) = crossbeam_channel::bounded(0);
        tx.send(completion_tx).ok();
        completion_rx.recv().ok();
        thread::sleep(Duration::from_millis(100));
    });

    rx
});

pub async fn make_request<'de, R>(request: &R) -> R::Response
where
    R: Request<false> + Send + Sync,
    R::Response: AsRef<Bytes>,
{
    let client = TestClient::new();

    let response = client
        .execute(request, Some(Duration::from_secs(10)))
        .await
        .unwrap();

    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(response.as_ref()) {
        println!("{}", json);
    } else {
        println!("{:?}", response.as_ref());
    }

    response
}

pub async fn make_auth_request<'de, R, E>(client: &E, request: &R) -> R::Response
where
    R: Request<true> + Send + Sync,
    R::Response: AsRef<Bytes>,
    E: AuthExecutor<R>,
{
    let response = client
        .execute(request, Some(Duration::from_secs(10)))
        .await
        .unwrap();

    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(response.as_ref()) {
        println!("{}", json);
    } else {
        println!("{:?}", response.as_ref());
    }

    response
}

#[derive(Deserialize)]
pub struct TestConfig {
    pub subaccount: Option<String>,
    subaccount_private_key: Option<String>,
    subaccount_public_key: Option<String>,
    main_private_key: Option<String>,
    main_public_key: Option<String>,
    pub perform_auth_api_tests: bool,
    pub perform_order_placement_tests: bool,
}

impl TestConfig {
    fn new() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        Ok(Config::builder()
            .set_default("perform_auth_api_tests", false)?
            .set_default("perform_order_placement_tests", false)?
            .add_source(Environment::with_prefix("FTX"))
            .build()?
            .try_deserialize()?)
    }
}

#[derive(Clone)]
pub struct AuthTestClient(AuthClient);

impl AuthTestClient {
    pub fn new_for_main() -> Self {
        match (
            CONFIG.main_private_key.as_ref(),
            CONFIG.main_public_key.as_ref(),
        ) {
            (Some(private_key), Some(public_key)) => {
                if private_key.len() == 0 {
                    panic!("empty private key")
                }
                if public_key.len() == 0 {
                    panic!("empty public key")
                }

                let auth = Authenticator::new(public_key.into(), private_key.into(), None).unwrap();

                Self(AuthClient::new(auth))
            }
            _ => panic!("invalid config for auth client, private and public keys must be defined"),
        }
    }

    pub fn new_for_subaccount() -> Self {
        match (CONFIG.subaccount_private_key.as_ref(), CONFIG.subaccount_public_key.as_ref(), CONFIG.subaccount.as_ref()) {
            (Some(private_key), Some(public_key), Some(subaccount)) => {
                if private_key.len() == 0 {
                    panic!("empty private key")
                }
                if public_key.len() == 0 {
                    panic!("empty public key")
                }
                if subaccount.len() == 0 {
                    panic!("empty subaccount")
                }
                let auth = Authenticator::new(
                    public_key.into(),
                    private_key.into(),
                    Some(subaccount.into())
                )
                .unwrap();

                Self(AuthClient::new(auth))
            }
            _ => panic!("invalid config for auth client, subaccount and private and public keys must be defined"),
        }
    }
}

#[async_trait]
impl<R> AuthExecutor<R> for AuthTestClient
where
    R: Request<true> + Send + Sync,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error> {
        let _token = loop {
            if let Ok(s) = THROTTLER.try_recv() {
                break s;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        };
        AuthExecutor::execute(&self.0, request, timeout).await
    }
}

#[derive(Clone)]
pub struct TestClient(Client);

impl TestClient {
    pub fn new() -> Self {
        Self(Client::new())
    }
}

#[async_trait]
impl<R> Executor<R> for TestClient
where
    R: Request<false> + Send + Sync,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error> {
        let _token = loop {
            if let Ok(s) = THROTTLER.try_recv() {
                break s;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        };
        self.0.execute(request, timeout).await
    }
}
