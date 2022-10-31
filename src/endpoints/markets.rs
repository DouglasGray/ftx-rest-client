use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, num::NonZeroU8};

use crate::{
    data::{FtxDateTime, Price, Side, Size, UnixTimestamp, WindowLength},
    private::Sealed,
    Json, OptJson, QueryParams, Request,
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

response!(GetMarketsResponse, Vec<MarketPartial<'a>>);

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

response!(GetMarketResponse, MarketPartial<'a>);

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

response!(GetOrderBookResponse, OrderBookPartial<'a>);

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

response!(GetTradesResponse, Vec<TradePartial<'a>>);

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

response!(GetCandlesResponse, Vec<CandlePartial<'a>>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MarketPartial<'a> {
    pub name: &'a str,
    pub underlying: Option<&'a str>,
    pub base_currency: Option<&'a str>,
    pub quote_currency: Option<&'a str>,
    #[serde(borrow)]
    pub r#type: Json<'a, MarketType>,
    #[serde(borrow)]
    pub enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub ask: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub bid: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub last: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub post_only: Json<'a, bool>,
    #[serde(borrow)]
    pub price_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub size_increment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub min_provide_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub tokenized_equity: OptJson<'a, bool>,
    #[serde(borrow)]
    pub restricted: Json<'a, bool>,
    #[serde(borrow)]
    pub high_leverage_fee_exempt: OptJson<'a, bool>,
    #[serde(borrow)]
    pub price_high_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub price_low_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub change_1h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub change_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub change_bod: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub quote_volume_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub volume_usd_24h: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub large_order_threshold: Json<'a, Decimal>,
    #[serde(borrow)]
    pub is_etf_market: Json<'a, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OrderBookPartial<'a> {
    #[serde(borrow)]
    pub asks: Vec<(Json<'a, Price>, Json<'a, Size>)>,
    #[serde(borrow)]
    pub bids: Vec<(Json<'a, Price>, Json<'a, Size>)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TradePartial<'a> {
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    #[serde(borrow)]
    pub liquidation: Json<'a, bool>,
    #[serde(borrow)]
    pub price: Json<'a, Decimal>,
    #[serde(borrow)]
    pub side: Json<'a, Side>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CandlePartial<'a> {
    #[serde(borrow)]
    pub close: Json<'a, Decimal>,
    #[serde(borrow)]
    pub high: Json<'a, Decimal>,
    #[serde(borrow)]
    pub low: Json<'a, Decimal>,
    #[serde(borrow)]
    pub open: Json<'a, Decimal>,
    #[serde(borrow)]
    pub volume: Json<'a, Decimal>,
    #[serde(borrow)]
    pub start_time: Json<'a, FtxDateTime>,
    #[serde(borrow)]
    pub time: Json<'a, UnixTimestamp>,
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::Response;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Market<'a> {
        pub r#type: MarketType,
        pub name: &'a str,
        pub underlying: Option<&'a str>,
        pub base_currency: Option<&'a str>,
        pub quote_currency: Option<&'a str>,
        pub enabled: bool,
        pub ask: Option<Decimal>,
        pub bid: Option<Decimal>,
        pub last: Option<Decimal>,
        pub price: Option<Decimal>,
        pub post_only: bool,
        pub price_increment: Decimal,
        pub size_increment: Decimal,
        pub min_provide_size: Decimal,
        pub tokenized_equity: Option<bool>,
        pub restricted: bool,
        pub high_leverage_fee_exempt: Option<bool>,
        pub price_high_24h: Option<Decimal>,
        pub price_low_24h: Option<Decimal>,
        pub change_1h: Option<Decimal>,
        pub change_24h: Option<Decimal>,
        pub change_bod: Option<Decimal>,
        pub quote_volume_24h: Option<Decimal>,
        pub volume_usd_24h: Option<Decimal>,
        pub large_order_threshold: Decimal,
        pub is_etf_market: bool,
    }

    impl<'a> TryFrom<MarketPartial<'a>> for Market<'a> {
        type Error = serde_json::Error;

        fn try_from(val: MarketPartial<'a>) -> Result<Self, Self::Error> {
            Ok(Self {
                name: val.name,
                underlying: val.underlying,
                r#type: val.r#type.deserialize()?,
                enabled: val.enabled.deserialize()?,
                post_only: val.post_only.deserialize()?,
                price_increment: val.price_increment.deserialize()?,
                size_increment: val.size_increment.deserialize()?,
                last: val.last.deserialize()?,
                bid: val.bid.deserialize()?,
                ask: val.ask.deserialize()?,
                change_1h: val.change_1h.deserialize()?,
                change_24h: val.change_24h.deserialize()?,
                change_bod: val.change_bod.deserialize()?,
                volume_usd_24h: val.volume_usd_24h.deserialize()?,
                base_currency: val.base_currency,
                quote_currency: val.quote_currency,
                price: val.price.deserialize()?,
                min_provide_size: val.min_provide_size.deserialize()?,
                tokenized_equity: val.tokenized_equity.deserialize()?,
                restricted: val.restricted.deserialize()?,
                high_leverage_fee_exempt: val.high_leverage_fee_exempt.deserialize()?,
                price_high_24h: val.price_high_24h.deserialize()?,
                price_low_24h: val.price_low_24h.deserialize()?,
                quote_volume_24h: val.quote_volume_24h.deserialize()?,
                large_order_threshold: val.large_order_threshold.deserialize()?,
                is_etf_market: val.is_etf_market.deserialize()?,
            })
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct OrderBook {
        pub asks: Vec<(Price, Size)>,
        pub bids: Vec<(Price, Size)>,
    }

    impl<'a> TryFrom<OrderBookPartial<'a>> for OrderBook {
        type Error = serde_json::Error;

        fn try_from(val: OrderBookPartial<'a>) -> Result<Self, Self::Error> {
            let mut asks = Vec::with_capacity(val.asks.len());
            let mut bids = Vec::with_capacity(val.bids.len());

            for (p, s) in val.asks.into_iter() {
                asks.push((p.deserialize()?, s.deserialize()?));
            }
            for (p, s) in val.bids.into_iter() {
                bids.push((p.deserialize()?, s.deserialize()?));
            }

            Ok(Self { asks, bids })
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Trade {
        pub id: u64,
        pub liquidation: bool,
        pub price: Decimal,
        pub side: Side,
        pub size: Decimal,
        pub time: FtxDateTime,
    }

    impl<'a> TryFrom<TradePartial<'a>> for Trade {
        type Error = serde_json::Error;

        fn try_from(val: TradePartial<'a>) -> Result<Self, Self::Error> {
            Ok(Self {
                id: val.id.deserialize()?,
                liquidation: val.liquidation.deserialize()?,
                price: val.price.deserialize()?,
                side: val.side.deserialize()?,
                size: val.size.deserialize()?,
                time: val.time.deserialize()?,
            })
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Candle {
        pub close: Decimal,
        pub high: Decimal,
        pub low: Decimal,
        pub open: Decimal,
        pub volume: Decimal,
        pub start_time: FtxDateTime,
        pub time: UnixTimestamp,
    }

    impl<'a> TryFrom<CandlePartial<'a>> for Candle {
        type Error = serde_json::Error;

        fn try_from(val: CandlePartial<'a>) -> Result<Self, Self::Error> {
            Ok(Self {
                close: val.close.deserialize()?,
                high: val.high.deserialize()?,
                low: val.low.deserialize()?,
                open: val.open.deserialize()?,
                volume: val.volume.deserialize()?,
                start_time: val.start_time.deserialize()?,
                time: val.time.deserialize()?,
            })
        }
    }

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
        let _: Vec<Market> = GetMarketsResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Market::try_from(p).unwrap())
            .collect();
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
        let _: Market<'_> = GetMarketResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .try_into()
            .unwrap();
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
        let _: Vec<Trade> = GetTradesResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Trade::try_from(p).unwrap())
            .collect();
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
        let _: Vec<Candle> = GetCandlesResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Candle::try_from(p).unwrap())
            .collect();
    }
}
