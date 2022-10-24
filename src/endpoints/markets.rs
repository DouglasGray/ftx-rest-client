use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, num::NonZeroU8};

use crate::{
    data::{
        DateTimeStr, NonNegativeDecimal, PositiveDecimal, Price, Side, Size, UnixTimestamp,
        WindowLength,
    },
    private::Sealed,
    QueryParams, Request,
};

use super::macros::response;

macro_rules! get_market_path {
    () => {
        "/markets/{market}"
    };
}

macro_rules! get_orderbook_path {
    () => {
        "/markets/{market}/orderbook"
    };
}

macro_rules! get_trades_path {
    () => {
        "/markets/{market}/trades"
    };
}

macro_rules! get_candles_path {
    () => {
        "/markets/{market}/candles"
    };
}

/// Market type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketType {
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "spot")]
    Spot,
}

/// Number of orderbook levels to return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookDepth(u8);

impl BookDepth {
    pub fn new(depth: NonZeroU8) -> Option<Self> {
        if depth.get() > 100 {
            None
        } else {
            Some(BookDepth(depth.get()))
        }
    }
}

/// Retrieve info on all markets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetMarkets;

impl Sealed for GetMarkets {}

impl Request<false> for GetMarkets {
    const PATH: &'static str = "/markets";

    const METHOD: Method = Method::GET;

    type Response = GetMarketsResponse;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMarketsResponse(Bytes);

response!(GetMarketsResponse, Vec<Market<'a>>);

/// Retrieve info on a single market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetMarket<'a> {
    pub market: &'a str,
}

impl<'a> Sealed for GetMarket<'a> {}

impl<'a> Request<false> for GetMarket<'a> {
    const PATH: &'static str = get_market_path!();

    const METHOD: Method = Method::GET;

    type Response = GetMarketResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_market_path!(), market = self.market))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMarketResponse(Bytes);

response!(GetMarketResponse, Market<'a>);

/// Retrieve an orderbook snapshot for the provided market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetOrderBook<'a> {
    pub market: &'a str,
    pub depth: Option<BookDepth>,
}

impl<'a> Sealed for GetOrderBook<'a> {}

impl<'a> Request<false> for GetOrderBook<'a> {
    const PATH: &'static str = get_orderbook_path!();

    const METHOD: Method = Method::GET;

    type Response = GetOrderBookResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_orderbook_path!(), market = self.market))
    }

    fn query_params(&self) -> Option<QueryParams> {
        self.depth.map(|d| vec![("depth", d.0.to_string())])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetOrderBookResponse(Bytes);

response!(GetOrderBookResponse, OrderBook);

/// Retrieve trades in some time frame for the provided market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetTrades<'a> {
    pub market: &'a str,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl<'a> Sealed for GetTrades<'a> {}

impl<'a> Request<false> for GetTrades<'a> {
    const PATH: &'static str = get_trades_path!();

    const METHOD: Method = Method::GET;

    type Response = GetTradesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(get_trades_path!(), market = self.market))
    }

    fn query_params(&self) -> Option<QueryParams> {
        if self.start_time.is_none() && self.end_time.is_none() {
            return None;
        }

        let mut params = Vec::with_capacity(2);

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
pub struct GetTradesResponse(Bytes);

response!(GetTradesResponse, Vec<Trade<'a>>);

/// Retrieve historical prices in some time frame for the provided
/// market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetCandles<'a> {
    pub market: &'a str,
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
        Cow::Owned(format!(get_candles_path!(), market = self.market))
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

response!(GetCandlesResponse, Vec<Candle<'a>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Market<'a> {
    pub r#type: MarketType,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub underlying: Option<&'a str>,
    #[serde(borrow)]
    pub base_currency: Option<&'a str>,
    #[serde(borrow)]
    pub quote_currency: Option<&'a str>,
    pub enabled: bool,
    pub ask: Option<Price>,
    pub bid: Option<Price>,
    pub last: Option<Price>,
    pub price: Option<Price>,
    pub post_only: bool,
    pub price_increment: PositiveDecimal,
    pub size_increment: PositiveDecimal,
    pub min_provide_size: PositiveDecimal,
    pub tokenized_equity: Option<bool>,
    pub restricted: bool,
    pub high_leverage_fee_exempt: Option<bool>,
    pub price_high_24h: Option<Price>,
    pub price_low_24h: Option<Price>,
    pub change_1h: Option<Decimal>,
    pub change_24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub quote_volume_24h: Option<NonNegativeDecimal>,
    pub volume_usd_24h: Option<NonNegativeDecimal>,
    pub large_order_threshold: Size,
    pub is_etf_market: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OrderBook {
    pub asks: Vec<(Price, Size)>,
    pub bids: Vec<(Price, Size)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Trade<'a> {
    pub id: u64,
    pub liquidation: bool,
    pub price: Price,
    pub side: Side,
    pub size: Size,
    #[serde(borrow)]
    pub time: DateTimeStr<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Candle<'a> {
    pub close: Price,
    pub high: Price,
    pub low: Price,
    pub open: Price,
    pub volume: NonNegativeDecimal,
    #[serde(borrow)]
    pub start_time: DateTimeStr<'a>,
    #[serde(deserialize_with = "super::float_ts_to_unix_ts")]
    pub time: UnixTimestamp,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_markets() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "name": "BTC-PERP",
      "baseCurrency": null,
      "quoteCurrency": null,
      "quoteVolume24h": 28914.76,
      "change1h": 0.012,
      "change24h": 0.0299,
      "changeBod": 0.0156,
      "highLeverageFeeExempt": false,
      "minProvideSize": 0.001,
      "type": "future",
      "underlying": "BTC",
      "enabled": true,
      "ask": 3949.25,
      "bid": 3949,
      "last": 10579.52,
      "postOnly": false,
      "price": 10579.52,
      "priceIncrement": 0.25,
      "sizeIncrement": 0.0001,
      "restricted": false,
      "volumeUsd24h": 28914.76,
      "largeOrderThreshold": 5000.0,
      "isEtfMarket": false
    }
  ]
}
"#;
        GetMarketsResponse(json.as_bytes().into()).parse().unwrap();
    }

    #[test]
    fn get_market() {
        let json = r#"
{
  "success": true,
  "result": {
      "name": "BTC-PERP",
      "baseCurrency": null,
      "quoteCurrency": null,
      "quoteVolume24h": 28914.76,
      "change1h": 0.012,
      "change24h": 0.0299,
      "changeBod": 0.0156,
      "highLeverageFeeExempt": false,
      "minProvideSize": 0.001,
      "type": "future",
      "underlying": "BTC",
      "enabled": true,
      "ask": 3949.25,
      "bid": 3949,
      "last": 10579.52,
      "postOnly": false,
      "price": 10579.52,
      "priceIncrement": 0.25,
      "sizeIncrement": 0.0001,
      "restricted": false,
      "volumeUsd24h": 28914.76,
      "largeOrderThreshold": 5000.0,
      "isEtfMarket": false
    }
}
"#;
        GetMarketResponse(json.as_bytes().into()).parse().unwrap();
    }

    #[test]
    fn get_trades() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "id": 3855995,
      "liquidation": false,
      "price": 3857.75,
      "side": "buy",
      "size": 0.111,
      "time": "2019-03-20T18:16:23.397991+00:00"
    }
  ]
}
"#;

        GetTradesResponse(json.as_bytes().into()).parse().unwrap();
    }

    #[test]
    fn get_candles() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "startTime": "2022-04-03T14:43:00+00:00",
      "time": 1648996980000,
      "open": 46371,
      "high": 46381,
      "low": 46371,
      "close": 46380,
      "volume": 1051438.0941
    }
  ]
}
"#;
        GetCandlesResponse(json.as_bytes().into()).parse().unwrap();
    }
}
