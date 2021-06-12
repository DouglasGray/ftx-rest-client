use async_trait::async_trait;
use bytes::Bytes;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use std::{borrow::Cow, convert::TryInto, error::Error as StdError, fmt, time::Duration};
use time::OffsetDateTime;

use crate::{
    auth::Authenticator,
    error::{BoxError, Error, ErrorKind},
    AuthExecutor, Executor, QueryParams, Request,
};

const BASE_URL: &str = "https://ftx.com/api";

#[derive(Clone)]
pub struct Client(reqwest::Client);

impl Client {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }
}

#[async_trait]
impl<R> Executor<R> for Client
where
    R: Request<false> + Send + Sync,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error> {
        build_and_execute_request(request, timeout, &self.0, None).await
    }
}

#[derive(Clone)]
pub struct AuthClient {
    authenticator: Authenticator,
    executor: reqwest::Client,
}

impl AuthClient {
    pub fn new(authenticator: Authenticator) -> Self {
        Self {
            authenticator,
            executor: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl<R> Executor<R> for AuthClient
where
    R: Request<false> + Send + Sync,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error> {
        build_and_execute_request(request, timeout, &self.executor, None).await
    }
}

#[async_trait]
impl<R> AuthExecutor<R> for AuthClient
where
    R: Request<true> + Send + Sync,
{
    async fn execute(&self, request: &R, timeout: Option<Duration>) -> Result<R::Response, Error> {
        build_and_execute_request(request, timeout, &self.executor, Some(&self.authenticator)).await
    }
}

async fn build_and_execute_request<R, const AUTH: bool>(
    request: &R,
    timeout: Option<Duration>,
    executor: &reqwest::Client,
    authenticator: Option<&Authenticator>,
) -> Result<R::Response, Error>
where
    R: Request<AUTH>,
{
    let path = request.path();

    let path_with_params = build_path_with_params(&path, request.query_params().as_ref())?;

    let url = format!("{}{}", BASE_URL, path_with_params.as_ref());

    let mut builder = executor.request(R::METHOD, url);

    if let Some(t) = timeout {
        builder = builder.timeout(t);
    }

    let payload = if let Some(res) = request.to_json() {
        Some(res.map_err(|e| Error::new(ErrorKind::InvalidPayload).with_source(e))?)
    } else {
        None
    };

    if let Some(authenticator) = authenticator {
        let headers = authenticator.generate_auth_headers(
            OffsetDateTime::now_utc()
                .try_into()
                .expect("timestamp will be > 0"),
            &R::METHOD,
            &path_with_params,
            payload.as_ref().map(String::as_str),
        )?;

        builder = builder.headers(headers);
    }

    if let Some(payload) = payload {
        builder = builder
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(payload);
    }

    let req = builder
        .build()
        .map_err(|e| Error::new(ErrorKind::RequestBuildFailed).with_source(e))?;

    Ok(execute_request::<R::Response>(executor, req).await?)
}

fn build_path_with_params<'a>(
    path: &'a str,
    query_params: Option<&QueryParams>,
) -> Result<Cow<'a, str>, Error> {
    if let Some(params) = query_params {
        let query_string = serde_urlencoded::to_string(params).map_err(|e| {
            let error = BuildUrlError::new(
                format!(
                    "error generating query string from params {:?}",
                    query_params
                ),
                e,
            );
            Error::new(ErrorKind::InvalidUrl).with_source(error)
        })?;

        if !query_string.is_empty() {
            let mut path: String = path.into();
            path.push('?');
            path.push_str(&query_string);
            Ok(Cow::Owned(path))
        } else {
            Ok(Cow::Borrowed(path))
        }
    } else {
        Ok(Cow::Borrowed(path))
    }
}

async fn execute_request<T>(client: &reqwest::Client, request: reqwest::Request) -> Result<T, Error>
where
    T: From<Bytes>,
{
    Ok(client
        .execute(request)
        .await
        .map_err(|e| Error::from_status_code(e.status()).with_source(e))?
        .bytes()
        .await
        .map_err(|e| Error::from_status_code(e.status()).with_source(e))?
        .into())
}

#[derive(Debug)]
struct BuildUrlError(String, BoxError);

impl BuildUrlError {
    fn new(reason: String, source: impl Into<BoxError>) -> Self {
        Self(reason, source.into())
    }
}

impl fmt::Display for BuildUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for BuildUrlError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.1.as_ref())
    }
}
