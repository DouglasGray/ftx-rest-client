use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    data::{DateTimeStr, NonNegativeDecimal, Price, Side, Size, SortOrder, UnixTimestamp},
    private::Sealed,
    QueryParams, Request,
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

response!(GetFillsResponse, Vec<Fill<'a>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Fill<'a> {
    pub market: &'a str,
    pub future: Option<&'a str>,
    pub side: Side,
    pub price: Price,
    pub size: Size,
    pub time: DateTimeStr<'a>,
    pub id: u64,
    pub order_id: u64,
    pub trade_id: u64,
    pub base_currency: Option<&'a str>,
    pub quote_currency: Option<&'a str>,
    pub r#type: FillType,
    pub liquidity: FillLiquidityType,
    pub fee: NonNegativeDecimal,
    pub fee_currency: &'a str,
    pub fee_rate: NonNegativeDecimal,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

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
        GetFillsResponse(json.as_bytes().into()).parse().unwrap();
    }
}
