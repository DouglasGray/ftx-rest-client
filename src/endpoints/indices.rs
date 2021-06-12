use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

use crate::{
    data::{
        BaseCurrency, DateTimeStr, Exchange, NonNegativeDecimal, Price, QuoteCurrency, Underlying,
        UnixTimestamp, WindowLength,
    },
    private::Sealed,
    QueryParams, Request,
};

use super::macros::response;

macro_rules! get_weights_path {
    () => {
        "/indexes/{index}/weights"
    };
}

macro_rules! get_candles_path {
    () => {
        "/indexes/{index}/candles"
    };
}

macro_rules! get_constituents_path {
    () => {
        "/index_constituents/{underlying}"
    };
}

/// Retrieve info on an index's composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWeights<'a> {
    pub index: &'a str,
}

impl<'a> Sealed for GetWeights<'a> {}

impl<'a> Request<false> for GetWeights<'a> {
    const PATH: &'static str = get_weights_path!();

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetWeightsResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_weights_path!(), index = self.index))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWeightsResponse(Bytes);

response!(
    GetWeightsResponse,
    HashMap<Underlying<'de>, NonNegativeDecimal>
);

/// Retrieve historical index prices in some time frame for the
/// provided futures market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetCandles<'a> {
    pub index: &'a str,
    pub resolution: WindowLength,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl<'a> Sealed for GetCandles<'a> {}

impl<'a> Request<false> for GetCandles<'a> {
    const PATH: &'static str = get_candles_path!();

    const METHOD: Method = Method::GET;

    type Response = GetCandlesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_candles_path!(), index = self.index))
    }

    fn query_params(&self) -> Option<QueryParams> {
        let mut params = Vec::with_capacity(3);

        params.push(("resolution", self.resolution.to_secs().to_string()));

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
pub struct GetCandlesResponse(Bytes);

response!(GetCandlesResponse, Vec<Candle<'de>>);

/// Retrieve information on an index's constituents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetConstituents<'a> {
    pub underlying: &'a str,
}

impl<'a> Sealed for GetConstituents<'a> {}

impl<'a> Request<false> for GetConstituents<'a> {
    const PATH: &'static str = get_constituents_path!();

    const METHOD: reqwest::Method = Method::GET;

    type Response = GetConstituentsResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            get_constituents_path!(),
            underlying = self.underlying
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConstituentsResponse(Bytes);

response!(
    GetConstituentsResponse,
    Vec<(Exchange<'de>, BaseCurrency<'de>, QuoteCurrency<'de>)>
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Candle<'a> {
    pub close: Price,
    pub high: Price,
    pub low: Price,
    pub open: Price,
    #[serde(borrow)]
    pub start_time: DateTimeStr<'a>,
    #[serde(deserialize_with = "super::float_ts_to_unix_ts")]
    pub time: UnixTimestamp,
    volume: Option<()>, // Is always `null`
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_weights() {
        let json = r#"
{
  "success": true,
  "result": {
    "BCH": 0.3492,
    "BNB": 2.8632,
    "BSV": 0.3471,
    "EOS": 18.1707,
    "ETH": 0.5724,
    "LTC": 1.2973,
    "XRP": 573.6345
  }
}
"#;
        GetWeightsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_candles() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "startTime": "2022-04-03T15:31:00+00:00",
      "time": 1648999860000,
      "open": 3999.0789733744436,
      "high": 3999.0789733744436,
      "low": 3996.910735872727,
      "close": 3996.910735872727,
      "volume": null
    }
  ]
}
"#;
        GetCandlesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_constituents() {
        let json = r#"
{
  "success": true,
  "result": [
    ["binance","BTC","TUSD"],
    ["bitstamp","BTC","USD"],
    ["bittrex","BTC","USD"]
  ]
}
"#;
        GetConstituentsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
