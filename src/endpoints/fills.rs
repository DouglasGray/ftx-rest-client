use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{FtxDateTime, Side, SortOrder, UnixTimestamp},
    private::Sealed,
    Json, QueryParams, Request,
};

use super::macros::response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FillType {
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "otc")]
    OTC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FillLiquidityType {
    #[serde(rename = "taker")]
    Taker,
    #[serde(rename = "maker")]
    Maker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFills<'a> {
    pub market: Option<&'a str>,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
    pub order_id: Option<u64>,
    pub order: Option<SortOrder>,
}

impl<'a> Sealed for GetFills<'a> {}

impl<'a> Request<true> for GetFills<'a> {
    const PATH: &'static str = "/fills";

    const METHOD: Method = Method::GET;

    type Response = GetFillsResponse;

    fn query_params(&self) -> Option<QueryParams> {
        if self.market.is_none()
            && self.start_time.is_none()
            && self.end_time.is_none()
            && self.order_id.is_none()
            && self.order.is_none()
        {
            return None;
        }

        let mut params = Vec::with_capacity(5);

        if let Some(market) = self.market {
            params.push(("market", market.into()))
        }
        if let Some(start_time) = self.start_time {
            params.push(("start_time", start_time.get().to_string()))
        }
        if let Some(end_time) = self.end_time {
            params.push(("end_time", end_time.get().to_string()))
        }
        if let Some(order_id) = self.order_id {
            params.push(("orderId", order_id.to_string()))
        }
        if matches!(self.order, Some(SortOrder::Ascending)) {
            params.push(("order", "asc".into()))
        }

        Some(params)
    }
}

pub struct GetFillsResponse(Bytes);

response!(GetFillsResponse, Vec<FillPartial<'a>>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FillPartial<'a> {
    pub market: &'a str,
    pub future: Option<&'a str>,
    #[serde(borrow)]
    pub side: Json<'a, Side>,
    #[serde(borrow)]
    pub price: Json<'a, Decimal>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    #[serde(borrow)]
    pub order_id: Json<'a, u64>,
    #[serde(borrow)]
    pub trade_id: Json<'a, u64>,
    pub base_currency: Option<&'a str>,
    pub quote_currency: Option<&'a str>,
    #[serde(borrow)]
    pub r#type: Json<'a, FillType>,
    #[serde(borrow)]
    pub liquidity: Json<'a, FillLiquidityType>,
    #[serde(borrow)]
    pub fee: Json<'a, Decimal>,
    pub fee_currency: &'a str,
    #[serde(borrow)]
    pub fee_rate: Json<'a, Decimal>,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::Response;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    struct Fill<'a> {
        pub market: &'a str,
        pub future: Option<&'a str>,
        pub side: Side,
        pub price: Decimal,
        pub size: Decimal,
        pub time: FtxDateTime,
        pub id: u64,
        pub order_id: u64,
        pub trade_id: u64,
        pub base_currency: Option<&'a str>,
        pub quote_currency: Option<&'a str>,
        pub r#type: FillType,
        pub liquidity: FillLiquidityType,
        pub fee: Decimal,
        pub fee_currency: &'a str,
        pub fee_rate: Decimal,
    }

    impl<'a> TryFrom<FillPartial<'a>> for Fill<'a> {
        type Error = serde_json::Error;

        fn try_from(val: FillPartial<'a>) -> Result<Self, Self::Error> {
            Ok(Fill {
                market: val.market,
                future: val.future,
                side: val.side.deserialize()?,
                price: val.price.deserialize()?,
                size: val.size.deserialize()?,
                time: val.time.deserialize()?,
                id: val.id.deserialize()?,
                order_id: val.order_id.deserialize()?,
                trade_id: val.trade_id.deserialize()?,
                base_currency: val.base_currency,
                quote_currency: val.quote_currency,
                r#type: val.r#type.deserialize()?,
                liquidity: val.liquidity.deserialize()?,
                fee: val.fee.deserialize()?,
                fee_currency: val.fee_currency,
                fee_rate: val.fee_rate.deserialize()?,
            })
        }
    }

    #[test]
    fn get_fills() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "fee": 20.1374935,
      "feeCurrency": "USD",
      "feeRate": 0.0005,
      "future": "EOS-0329",
      "id": 11215,
      "liquidity": "taker",
      "market": "EOS-0329",
      "baseCurrency": null,
      "quoteCurrency": null,
      "orderId": 8436981,
      "tradeId": 1013912,
      "price": 4.201,
      "side": "buy",
      "size": 9587,
      "time": "2019-03-27T19:15:10.204619+00:00",
      "type": "order"
    }
  ]
}
"#;
        let _: Vec<Fill<'_>> = GetFillsResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Fill::try_from(p).unwrap())
            .collect();
    }
}
