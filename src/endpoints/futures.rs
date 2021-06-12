use std::borrow::Cow;

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{DateTimeStr, NonNegativeDecimal, PositiveDecimal, Price, UnixTimestamp},
    private::Sealed,
    Request,
};

use super::macros::response;

macro_rules! get_future_path {
    () => {
        "/futures/{future}"
    };
}

macro_rules! get_future_stats_path {
    () => {
        "/futures/{future}/stats"
    };
}

/// Future type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutureType {
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "prediction")]
    Prediction,
}

/// Particular group a future may belong to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutureGroup {
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "prediction")]
    Prediction,
}

/// Retrieve information on all futures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFutures;

impl Sealed for GetFutures {}

impl Request<false> for GetFutures {
    const PATH: &'static str = "/futures";

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetFuturesResponse;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFuturesResponse(Bytes);

response!(GetFuturesResponse, Vec<Future<'de>>);

/// Retrieve information on a single future.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFuture<'a> {
    pub future: &'a str,
}

impl<'a> Sealed for GetFuture<'a> {}

impl<'a> Request<false> for GetFuture<'a> {
    const PATH: &'static str = get_future_path!();

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetFutureResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_future_path!(), future = self.future))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFutureResponse(Bytes);

response!(GetFutureResponse, Future<'de>);

/// Retrieve future statistics, including predicted funding rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFutureStats<'a> {
    pub future: &'a str,
}

impl<'a> Sealed for GetFutureStats<'a> {}

impl<'a> Request<false> for GetFutureStats<'a> {
    const PATH: &'static str = get_future_stats_path!();

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetFutureStatsResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_future_stats_path!(), future = self.future))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFutureStatsResponse(Bytes);

response!(GetFutureStatsResponse, FutureStats<'de>);

/// Retrieve historical funding rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFundingRates<'a> {
    pub perpetual: Option<&'a str>,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl<'a> Sealed for GetFundingRates<'a> {}

impl<'a> Request<false> for GetFundingRates<'a> {
    const PATH: &'static str = "/funding_rates";

    const METHOD: Method = Method::GET;

    type Response = GetFundingRatesResponse;

    fn query_params(&self) -> Option<crate::QueryParams> {
        if self.perpetual.is_none() && self.start_time.is_none() && self.end_time.is_none() {
            return None;
        }

        let mut params = Vec::with_capacity(3);

        if let Some(future) = self.perpetual {
            params.push(("future", future.into()));
        }
        if let Some(start_time) = self.start_time {
            params.push(("start_time", start_time.get().to_string()));
        }
        if let Some(end_time) = self.end_time {
            params.push(("end_time", end_time.get().to_string()));
        }

        Some(params)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFundingRatesResponse(Bytes);

response!(GetFundingRatesResponse, Vec<FundingRate<'de>>);

/// Retrieve information on all expired futures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetExpiredFutures;

impl Sealed for GetExpiredFutures {}

impl Request<false> for GetExpiredFutures {
    const PATH: &'static str = "/expired_futures";

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetExpiredFuturesResponse;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetExpiredFuturesResponse(Bytes);

response!(GetExpiredFuturesResponse, Vec<ExpiredFuture<'de>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Future<'a> {
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub underlying: &'a str,
    #[serde(borrow)]
    pub description: &'a str,
    #[serde(borrow)]
    pub underlying_description: &'a str,
    #[serde(borrow)]
    pub expiry_description: &'a str,
    pub r#type: FutureType,
    pub group: FutureGroup,
    #[serde(borrow)]
    pub expiry: Option<DateTimeStr<'a>>,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub price_increment: PositiveDecimal,
    pub size_increment: PositiveDecimal,
    pub last: Option<Price>,
    pub bid: Option<Price>,
    pub ask: Option<Price>,
    pub index: Option<Price>,
    pub mark: Option<Price>,
    pub imf_factor: PositiveDecimal,
    pub lower_bound: Option<PositiveDecimal>,
    pub upper_bound: Option<PositiveDecimal>,
    pub margin_price: Option<PositiveDecimal>,
    pub position_limit_weight: PositiveDecimal,
    pub change_1h: Option<Decimal>,
    pub change_24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub volume_usd_24h: NonNegativeDecimal,
    pub volume: NonNegativeDecimal,
    pub open_interest: NonNegativeDecimal,
    pub open_interest_usd: NonNegativeDecimal,
    #[serde(borrow)]
    pub move_start: Option<DateTimeStr<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FutureStats<'a> {
    pub volume: NonNegativeDecimal,
    pub next_funding_rate: Option<Decimal>,
    #[serde(borrow)]
    pub next_funding_time: DateTimeStr<'a>,
    pub expiration_price: Option<Price>,
    pub predicted_expiration_price: Option<Price>,
    pub strike_price: Option<Price>,
    pub open_interest: NonNegativeDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundingRate<'a> {
    #[serde(borrow)]
    pub future: &'a str,
    pub rate: Decimal,
    #[serde(borrow)]
    pub time: DateTimeStr<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ExpiredFuture<'a> {
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub underlying: &'a str,
    #[serde(borrow)]
    pub description: &'a str,
    #[serde(borrow)]
    pub underlying_description: &'a str,
    #[serde(borrow)]
    pub expiry_description: &'a str,
    pub r#type: FutureType,
    pub group: FutureGroup,
    #[serde(borrow)]
    pub expiry: Option<DateTimeStr<'a>>,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub price_increment: PositiveDecimal,
    pub size_increment: PositiveDecimal,
    pub last: Option<Price>,
    pub bid: Option<Price>,
    pub ask: Option<Price>,
    pub index: Option<Price>,
    pub mark: Option<Price>,
    pub imf_factor: PositiveDecimal,
    pub lower_bound: Option<PositiveDecimal>,
    pub upper_bound: Option<PositiveDecimal>,
    pub margin_price: Option<PositiveDecimal>,
    pub position_limit_weight: PositiveDecimal,
    #[serde(borrow)]
    pub move_start: Option<DateTimeStr<'a>>,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_futures() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "name": "BTC-MOVE-0402",
      "underlying": "BTC",
      "description": "Bitcoin MOVE 2022-04-02 Contracts",
      "type": "move",
      "expiry": "2022-04-03T00:00:00+00:00",
      "perpetual": false,
      "expired": false,
      "enabled": true,
      "postOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": 299,
      "bid": 294,
      "ask": 304,
      "index": 46088.731248179,
      "mark": 299,
      "imfFactor": 0.002,
      "lowerBound": 1,
      "upperBound": 4881,
      "underlyingDescription": "Bitcoin",
      "expiryDescription": "Today",
      "moveStart": "2022-04-02T00:00:00+00:00",
      "marginPrice": 46088.731248179,
      "positionLimitWeight": 2,
      "group": "daily",
      "change1h": 0.31140350877192985,
      "change24h": -0.6210392902408112,
      "changeBod": -0.6238993710691824,
      "volumeUsd24h": 361892.0658,
      "volume": 566.0078,
      "openInterest": 507.2044,
      "openInterestUsd": 151654.1156
    }
  ]
}
"#;
        GetFuturesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_future() {
        let json = r#"
{
  "success": true,
  "result": {
      "name": "BTC-MOVE-0402",
      "underlying": "BTC",
      "description": "Bitcoin MOVE 2022-04-02 Contracts",
      "type": "move",
      "expiry": "2022-04-03T00:00:00+00:00",
      "perpetual": false,
      "expired": false,
      "enabled": true,
      "postOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": 299,
      "bid": 294,
      "ask": 304,
      "index": 46088.731248179,
      "mark": 299,
      "imfFactor": 0.002,
      "lowerBound": 1,
      "upperBound": 4881,
      "underlyingDescription": "Bitcoin",
      "expiryDescription": "Today",
      "moveStart": "2022-04-02T00:00:00+00:00",
      "marginPrice": 46088.731248179,
      "positionLimitWeight": 2,
      "group": "daily",
      "change1h": 0.31140350877192985,
      "change24h": -0.6210392902408112,
      "changeBod": -0.6238993710691824,
      "volumeUsd24h": 361892.0658,
      "volume": 566.0078,
      "openInterest": 507.2044,
      "openInterestUsd": 151654.1156
    }
}
"#;
        GetFutureResponse(json.as_bytes().into()).to_data().unwrap();
    }

    #[test]
    fn future_stats() {
        let json = r#"
{
  "success": true,
  "result": {
    "volume": 1000.23,
    "nextFundingRate": 0.00025,
    "nextFundingTime": "2019-03-29T03:00:00+00:00",
    "expirationPrice": 3992.1,
    "predictedExpirationPrice": 3993.6,
    "strikePrice": 8182.35,
    "openInterest": 21124.583
  }
}
"#;
        GetFutureStatsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn funding_rates() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "future": "BTC-PERP",
      "rate": 0.0025,
      "time": "2019-06-02T08:00:00+00:00"
    }
  ]
}
"#;
        GetFundingRatesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_expired_futures() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "name": "BTC-MOVE-WK-0401",
      "underlying": "BTC",
      "description": "Bitcoin Weekly MOVE 2022-04-01 Contracts",
      "type": "move",
      "expiry": "2022-04-02T00:00:00+00:00",
      "perpetual": false,
      "expired": true,
      "enabled": false,
      "postOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": null,
      "bid": null,
      "ask": null,
      "index": 46300.342396571,
      "mark": 1883.99287306601,
      "imfFactor": 0.002,
      "lowerBound": 1,
      "upperBound": 6561,
      "underlyingDescription": "Bitcoin",
      "expiryDescription": "2022-04-01",
      "moveStart": "2022-03-26T00:00:00+00:00",
      "marginPrice": 46300.342396571,
      "positionLimitWeight": 2,
      "group": "weekly"
    }
  ]
}
"#;
        GetExpiredFuturesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
