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
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

    type PartialData<'a>
    where
        Self: 'a;

    fn deserialize<'a: 'de, 'de>(&'a self) -> Result<Self::Data<'a>, Error>
    where
        <Self as Response>::Data<'a>: Deserialize<'de>,
    {
        FtxResponse::try_from(self.as_ref().as_ref())?.deserialize()
    }

    fn deserialize_partial<'a: 'de, 'de>(&'a self) -> Result<Self::PartialData<'a>, Error>
    where
        <Self as Response>::PartialData<'a>: Deserialize<'de>,
    {
        FtxResponse::try_from(self.as_ref().as_ref())?.deserialize()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Json<'a, T> {
    val: &'a RawValue,
    _marker: PhantomData<T>,
}

impl<'a, T> Json<'a, T>
where
    T: Deserialize<'a>,
{
    pub fn deserialize(&self) -> serde_json::Result<T> {
        serde_json::from_str(self.val.get())
    }
}

impl<'a, T> From<&'a RawValue> for Json<'a, T> {
    fn from(val: &'a RawValue) -> Self {
        Self {
            val,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Serialize for Json<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.val.serialize(serializer)
    }
}

impl<'de: 'a, 'a, T> Deserialize<'de> for Json<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&'a RawValue>::deserialize(deserializer).map(Into::into)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OptJson<'a, T>(Option<Json<'a, T>>);

impl<'a, T> OptJson<'a, T>
where
    T: Deserialize<'a>,
{
    pub fn deserialize(&self) -> serde_json::Result<Option<T>> {
        self.0.as_ref().map(Json::deserialize).transpose()
    }
}

impl<'a, T> From<Option<Json<'a, T>>> for OptJson<'a, T> {
    fn from(value: Option<Json<'a, T>>) -> Self {
        Self(value)
    }
}

impl<'a, T> Serialize for OptJson<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(val) = &self.0 {
            val.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl<'de: 'a, 'a, T> Deserialize<'de> for OptJson<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<Json<'a, T>>::deserialize(deserializer).map(Into::into)
    }
}

mod private {
    pub trait Sealed {}
}
