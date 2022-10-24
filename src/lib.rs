pub mod error;
use error::Error;

mod client;
pub use client::{AuthClient, Client};

pub mod data;

pub mod endpoints;
use endpoints::FtxResponse;

pub mod auth;

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{borrow::Cow, convert::TryFrom, marker::PhantomData, time::Duration};

pub type QueryParams = Vec<(&'static str, String)>;

#[async_trait]
pub trait Executor<R>
where
    R: Request<false>,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error>;
}

#[async_trait]
pub trait AuthExecutor<R>
where
    R: Request<true>,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error>;
}

pub trait Request<const AUTH: bool>: private::Sealed {
    const PATH: &'static str;

    const METHOD: Method;

    type Response: Response;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }

    fn query_params(&self) -> Option<QueryParams> {
        None
    }

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        None
    }
}

pub trait Response: From<Bytes> + AsRef<Bytes> + private::Sealed {
    type Data<'a>
    where
        Self: 'a;

    fn parse<'a: 'de, 'de>(&'a self) -> Result<Self::Data<'a>, Error>
    where
        <Self as Response>::Data<'a>: Deserialize<'de>,
    {
        FtxResponse::try_from(self.as_ref().as_ref())?.try_into()
    }
}

mod private {
    pub trait Sealed {}
}
