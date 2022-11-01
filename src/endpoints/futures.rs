use std::{borrow::Cow, convert::TryFrom};

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{FtxDateTime, FutureType, UnixTimestamp},
    private::Sealed,
    Json, OptJson, Request,
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

response!(GetFuturesResponse, Vec<Future<'a>>, Vec<FuturePartial<'a>>);

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

response!(GetFutureResponse, Future<'a>, FuturePartial<'a>);

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

response!(GetFutureStatsResponse, FutureStats, FutureStatsPartial<'a>);

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

response!(
    GetFundingRatesResponse,
    Vec<FundingRate<'a>>,
    Vec<FundingRatePartial<'a>>
);

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

response!(
    GetExpiredFuturesResponse,
    Vec<ExpiredFuture<'a>>,
    Vec<ExpiredFuturePartial<'a>>
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Future<'a> {
    pub name: &'a str,
    pub underlying: &'a str,
    pub description: &'a str,
    pub underlying_description: &'a str,
    pub expiry_description: &'a str,
    pub r#type: FutureType,
    pub group: FutureGroup,
    pub expiry: Option<FtxDateTime>,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub close_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub last: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub index: Option<Decimal>,
    pub mark: Option<Decimal>,
    pub imf_factor: Decimal,
    pub imf_weight: Decimal,
    pub mmf_weight: Decimal,
    pub lower_bound: Option<Decimal>,
    pub upper_bound: Option<Decimal>,
    pub margin_price: Option<Decimal>,
    pub position_limit_weight: Decimal,
    pub change_1h: Option<Decimal>,
    pub change_24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub volume_usd_24h: Decimal,
    pub volume: Decimal,
    pub open_interest: Decimal,
    pub open_interest_usd: Decimal,
    pub move_start: Option<FtxDateTime>,
}

impl<'a> TryFrom<FuturePartial<'a>> for Future<'a> {
    type Error = serde_json::Error;

    fn try_from(val: FuturePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: val.name,
            underlying: val.underlying,
            description: val.description,
            underlying_description: val.underlying_description,
            expiry_description: val.expiry_description,
            r#type: val.r#type.deserialize()?,
            group: val.group.deserialize()?,
            expiry: val.expiry.deserialize()?,
            perpetual: val.perpetual.deserialize()?,
            expired: val.expired.deserialize()?,
            enabled: val.enabled.deserialize()?,
            post_only: val.post_only.deserialize()?,
            close_only: val.close_only.deserialize()?,
            price_increment: val.price_increment.deserialize()?,
            size_increment: val.size_increment.deserialize()?,
            last: val.last.deserialize()?,
            bid: val.bid.deserialize()?,
            ask: val.ask.deserialize()?,
            index: val.index.deserialize()?,
            mark: val.mark.deserialize()?,
            imf_factor: val.imf_factor.deserialize()?,
            imf_weight: val.imf_weight.deserialize()?,
            mmf_weight: val.mmf_weight.deserialize()?,
            lower_bound: val.lower_bound.deserialize()?,
            upper_bound: val.upper_bound.deserialize()?,
            margin_price: val.margin_price.deserialize()?,
            position_limit_weight: val.position_limit_weight.deserialize()?,
            change_1h: val.change_1h.deserialize()?,
            change_24h: val.change_24h.deserialize()?,
            change_bod: val.change_bod.deserialize()?,
            volume_usd_24h: val.volume_usd_24h.deserialize()?,
            volume: val.volume.deserialize()?,
            open_interest: val.open_interest.deserialize()?,
            open_interest_usd: val.open_interest_usd.deserialize()?,
            move_start: val.move_start.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FuturePartial<'a> {
    pub name: &'a str,
    pub underlying: &'a str,
    pub description: &'a str,
    pub underlying_description: &'a str,
    pub expiry_description: &'a str,
    #[serde(borrow)]
    pub r#type: Json<'a, FutureType>,
    #[serde(borrow)]
    pub group: Json<'a, FutureGroup>,
    #[serde(borrow)]
    pub expiry: OptJson<'a, FtxDateTime>,
    #[serde(borrow)]
    pub perpetual: Json<'a, bool>,
    #[serde(borrow)]
    pub expired: Json<'a, bool>,
    #[serde(borrow)]
    pub enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub post_only: Json<'a, bool>,
    #[serde(borrow)]
    pub close_only: Json<'a, bool>,
    #[serde(borrow)]
    pub price_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub size_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub last: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub bid: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub ask: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub index: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub mark: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub imf_factor: Json<'a, Decimal>,
    #[serde(borrow)]
    pub imf_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub mmf_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub lower_bound: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub upper_bound: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub margin_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub position_limit_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub change_1h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub change_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub change_bod: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub volume_usd_24h: Json<'a, Decimal>,
    #[serde(borrow)]
    pub volume: Json<'a, Decimal>,
    #[serde(borrow)]
    pub open_interest: Json<'a, Decimal>,
    #[serde(borrow)]
    pub open_interest_usd: Json<'a, Decimal>,
    #[serde(borrow)]
    pub move_start: OptJson<'a, FtxDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FutureStats {
    pub volume: Decimal,
    pub next_funding_rate: Option<Decimal>,
    pub next_funding_time: FtxDateTime,
    pub expiration_price: Option<Decimal>,
    pub predicted_expiration_price: Option<Decimal>,
    pub strike_price: Option<Decimal>,
    pub open_interest: Decimal,
}

impl<'a> TryFrom<FutureStatsPartial<'a>> for FutureStats {
    type Error = serde_json::Error;

    fn try_from(val: FutureStatsPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            volume: val.volume.deserialize()?,
            next_funding_rate: val.next_funding_rate.deserialize()?,
            next_funding_time: val.next_funding_time.deserialize()?,
            expiration_price: val.expiration_price.deserialize()?,
            predicted_expiration_price: val.predicted_expiration_price.deserialize()?,
            strike_price: val.strike_price.deserialize()?,
            open_interest: val.open_interest.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FutureStatsPartial<'a> {
    #[serde(borrow)]
    pub volume: Json<'a, Decimal>,
    #[serde(borrow)]
    pub next_funding_rate: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub next_funding_time: Json<'a, FtxDateTime>,
    #[serde(borrow)]
    pub expiration_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub predicted_expiration_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub strike_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub open_interest: Json<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundingRate<'a> {
    pub future: &'a str,
    pub rate: Decimal,
    pub time: FtxDateTime,
}

impl<'a> TryFrom<FundingRatePartial<'a>> for FundingRate<'a> {
    type Error = serde_json::Error;

    fn try_from(val: FundingRatePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            future: val.future,
            rate: val.rate.deserialize()?,
            time: val.time.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundingRatePartial<'a> {
    pub future: &'a str,
    #[serde(borrow)]
    pub rate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ExpiredFuture<'a> {
    pub name: &'a str,
    pub underlying: &'a str,
    pub description: &'a str,
    pub underlying_description: &'a str,
    pub expiry_description: &'a str,
    pub r#type: FutureType,
    pub group: FutureGroup,
    pub expiry: Option<FtxDateTime>,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub close_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub last: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub index: Option<Decimal>,
    pub index_adjustment: Option<Decimal>,
    pub mark: Option<Decimal>,
    pub imf_factor: Decimal,
    pub imf_weight: Decimal,
    pub mmf_weight: Decimal,
    pub lower_bound: Option<Decimal>,
    pub upper_bound: Option<Decimal>,
    pub margin_price: Option<Decimal>,
    pub position_limit_weight: Decimal,
    pub move_start: Option<FtxDateTime>,
}

impl<'a> TryFrom<ExpiredFuturePartial<'a>> for ExpiredFuture<'a> {
    type Error = serde_json::Error;

    fn try_from(val: ExpiredFuturePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: val.name,
            underlying: val.underlying,
            description: val.description,
            underlying_description: val.underlying_description,
            expiry_description: val.expiry_description,
            r#type: val.r#type.deserialize()?,
            group: val.group.deserialize()?,
            expiry: val.expiry.deserialize()?,
            perpetual: val.perpetual.deserialize()?,
            expired: val.expired.deserialize()?,
            enabled: val.enabled.deserialize()?,
            post_only: val.post_only.deserialize()?,
            close_only: val.close_only.deserialize()?,
            price_increment: val.price_increment.deserialize()?,
            size_increment: val.size_increment.deserialize()?,
            last: val.last.deserialize()?,
            bid: val.bid.deserialize()?,
            ask: val.ask.deserialize()?,
            index: val.index.deserialize()?,
            index_adjustment: val.index_adjustment.deserialize()?,
            mark: val.mark.deserialize()?,
            imf_factor: val.imf_factor.deserialize()?,
            imf_weight: val.imf_weight.deserialize()?,
            mmf_weight: val.mmf_weight.deserialize()?,
            lower_bound: val.lower_bound.deserialize()?,
            upper_bound: val.upper_bound.deserialize()?,
            margin_price: val.margin_price.deserialize()?,
            position_limit_weight: val.position_limit_weight.deserialize()?,
            move_start: val.move_start.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ExpiredFuturePartial<'a> {
    pub name: &'a str,
    pub underlying: &'a str,
    pub description: &'a str,
    pub underlying_description: &'a str,
    pub expiry_description: &'a str,
    #[serde(borrow)]
    pub r#type: Json<'a, FutureType>,
    #[serde(borrow)]
    pub group: Json<'a, FutureGroup>,
    #[serde(borrow)]
    pub expiry: OptJson<'a, FtxDateTime>,
    #[serde(borrow)]
    pub perpetual: Json<'a, bool>,
    #[serde(borrow)]
    pub expired: Json<'a, bool>,
    #[serde(borrow)]
    pub enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub post_only: Json<'a, bool>,
    #[serde(borrow)]
    pub close_only: Json<'a, bool>,
    #[serde(borrow)]
    pub price_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub size_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub last: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub bid: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub ask: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub index: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub index_adjustment: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub mark: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub imf_factor: Json<'a, Decimal>,
    #[serde(borrow)]
    pub imf_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub mmf_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub lower_bound: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub upper_bound: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub margin_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub position_limit_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub move_start: OptJson<'a, FtxDateTime>,
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

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
      "closeOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": 299,
      "bid": 294,
      "ask": 304,
      "index": 46088.731248179,
      "mark": 299,
      "imfFactor": 0.002,
      "imfWeight": 1,
      "mmfWeight": 1,
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
        let response = GetFuturesResponse(json.as_bytes().into());

        let from_partial: Vec<Future<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Future::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
      "closeOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": 299,
      "bid": 294,
      "ask": 304,
      "index": 46088.731248179,
      "mark": 299,
      "imfFactor": 0.002,
      "imfWeight": 1,
      "mmfWeight": 1,
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
        let response = GetFutureResponse(json.as_bytes().into());

        let from_partial: Future<'_> = response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = GetFutureStatsResponse(json.as_bytes().into());

        let from_partial: FutureStats = response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = GetFundingRatesResponse(json.as_bytes().into());

        let from_partial: Vec<FundingRate<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| FundingRate::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
      "closeOnly": false,
      "priceIncrement": 1,
      "sizeIncrement": 0.0001,
      "last": null,
      "bid": null,
      "ask": null,
      "index": 46300.342396571,
      "indexAdjustment": 0.342396571,
      "mark": 1883.99287306601,
      "imfFactor": 0.002,
      "imfWeight": 1,
      "mmfWeight": 1,
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
        let response = GetExpiredFuturesResponse(json.as_bytes().into());

        let from_partial: Vec<ExpiredFuture<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| ExpiredFuture::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }
}
