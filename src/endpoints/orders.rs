use std::{borrow::Cow, convert::TryFrom};

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{CancelAckMsg, FtxDateTime, PositiveDecimal, Side, UnixTimestamp},
    private::Sealed,
    Json, OptJson, QueryParams, Request,
};

use super::macros::response;

macro_rules! get_order_status_path {
    () => {
        "/orders/{order_id}"
    };
}

macro_rules! edit_order_path {
    () => {
        "/orders/{order_id}/modify"
    };
}

macro_rules! cancel_order_path {
    () => {
        "/orders/{order_id}"
    };
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}

impl OrderType {
    pub fn as_param(&self) -> &str {
        match self {
            Self::Limit => "limit",
            Self::Market => "market",
        }
    }
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

/// Type of order id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderId<'a> {
    /// An order id issued by the exchange.
    Exchange(u64),
    /// An order id requested by the client. Will always have a
    /// corresponding exchange issued id.
    Client(&'a str),
}

/// Order edit options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditOrderOpts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PositiveDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PositiveDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
}

/// Available order options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct OrderOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_on_price_band: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_after_ts: Option<UnixTimestamp>,
}

/// Retrieve all open orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetOpenOrders<'a> {
    pub market: Option<&'a str>,
}

impl<'a> Sealed for GetOpenOrders<'a> {}

impl<'a> Request<true> for GetOpenOrders<'a> {
    const PATH: &'static str = "/orders";

    const METHOD: Method = Method::GET;

    type Response = GetOpenOrdersResponse;

    fn query_params(&self) -> Option<QueryParams> {
        self.market.map(|m| vec![("market", m.into())])
    }
}

pub struct GetOpenOrdersResponse(Bytes);

response!(GetOpenOrdersResponse, Vec<Order<'a>>, Vec<OrderPartial<'a>>);

/// Retrieve information on historical orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetOrderHistory<'a> {
    pub market: Option<&'a str>,
    pub side: Option<Side>,
    pub order_type: Option<OrderType>,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl<'a> Sealed for GetOrderHistory<'a> {}

impl<'a> Request<true> for GetOrderHistory<'a> {
    const PATH: &'static str = "/orders/history";

    const METHOD: Method = Method::GET;

    type Response = GetOrderHistoryResponse;

    fn query_params(&self) -> Option<QueryParams> {
        if self.market.is_none()
            && self.side.is_none()
            && self.order_type.is_none()
            && self.start_time.is_none()
            && self.end_time.is_none()
        {
            return None;
        }

        let mut params = Vec::with_capacity(5);

        if let Some(market) = self.market {
            params.push(("market", market.into()));
        }
        if let Some(side) = self.side {
            params.push(("side", side.as_param().into()))
        }
        if let Some(order_type) = self.order_type {
            params.push(("orderType", order_type.as_param().into()))
        }
        if let Some(start_time) = self.start_time {
            params.push(("start_time", start_time.get().to_string()))
        }
        if let Some(end_time) = self.end_time {
            params.push(("end_time", end_time.get().to_string()))
        }

        Some(params)
    }
}

pub struct GetOrderHistoryResponse(Bytes);

response!(
    GetOrderHistoryResponse,
    Vec<Order<'a>>,
    Vec<OrderPartial<'a>>
);

/// Retrieve the status of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOrderStatus<'a> {
    pub order_id: OrderId<'a>,
}

impl<'a> Sealed for GetOrderStatus<'a> {}

impl<'a> Request<true> for GetOrderStatus<'a> {
    const PATH: &'static str = get_order_status_path!();

    const METHOD: Method = Method::GET;

    type Response = GetOrderStatusResponse;

    fn path(&self) -> Cow<'_, str> {
        let path = match self.order_id {
            OrderId::Exchange(id) => format!(get_order_status_path!(), order_id = id),
            OrderId::Client(id) => format!(
                get_order_status_path!(),
                order_id = format!("by_client_id/{}", id)
            ),
        };

        Cow::Owned(path)
    }
}

pub struct GetOrderStatusResponse(Bytes);

response!(GetOrderStatusResponse, Order<'a>, OrderPartial<'a>);

/// Place an order. Set price to `None` if submitting a market order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder<'a> {
    pub market: &'a str,
    pub side: Side,
    pub price: Option<PositiveDecimal>,
    pub size: PositiveDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub opts: Option<OrderOpts>,
}

impl<'a> Sealed for PlaceOrder<'a> {}

impl<'a> Request<true> for PlaceOrder<'a> {
    const PATH: &'static str = "/orders";

    const METHOD: Method = Method::POST;

    type Response = PlaceOrderResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct PlaceOrderResponse(Bytes);

response!(PlaceOrderResponse, OrderPlaced<'a>, OrderPlacedPartial<'a>);

/// Edit an order. Exchange side this behaves like a cancel followed
/// by a replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditOrder<'a> {
    pub order_id: OrderId<'a>,
    pub opts: EditOrderOpts<'a>,
}

impl<'a> Sealed for EditOrder<'a> {}

impl<'a> Request<true> for EditOrder<'a> {
    const PATH: &'static str = edit_order_path!();

    const METHOD: Method = Method::POST;

    type Response = EditOrderResponse;

    fn path(&self) -> Cow<'_, str> {
        let path = match self.order_id {
            OrderId::Exchange(id) => format!(edit_order_path!(), order_id = id),
            OrderId::Client(id) => format!(
                edit_order_path!(),
                order_id = format!("by_client_id/{}", id)
            ),
        };

        Cow::Owned(path)
    }

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(&self.opts))
    }
}

pub struct EditOrderResponse(Bytes);

response!(EditOrderResponse, OrderPlaced<'a>, OrderPlacedPartial<'a>);

/// Cancel an order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelOrder<'a> {
    pub order_id: OrderId<'a>,
}

impl<'a> Sealed for CancelOrder<'a> {}

impl<'a> Request<true> for CancelOrder<'a> {
    const PATH: &'static str = cancel_order_path!();

    const METHOD: Method = Method::DELETE;

    type Response = CancelOrderResponse;

    fn path(&self) -> Cow<'_, str> {
        let path = match self.order_id {
            OrderId::Exchange(id) => format!(cancel_order_path!(), order_id = id),
            OrderId::Client(id) => format!(
                cancel_order_path!(),
                order_id = format!("by_client_id/{}", id)
            ),
        };

        Cow::Owned(path)
    }
}

pub struct CancelOrderResponse(Bytes);

response!(CancelOrderResponse, CancelAckMsg<'a>, CancelAckMsg<'a>);

/// Cancel all orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrders<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_orders_only: Option<bool>,
}

impl<'a> Sealed for CancelAllOrders<'a> {}

impl<'a> Request<true> for CancelAllOrders<'a> {
    const PATH: &'static str = "/orders";

    const METHOD: Method = Method::DELETE;

    type Response = CancelAllOrdersResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct CancelAllOrdersResponse(Bytes);

response!(CancelAllOrdersResponse, CancelAckMsg<'a>, CancelAckMsg<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Order<'a> {
    pub id: u64,
    pub client_id: Option<&'a str>,
    pub market: &'a str,
    pub future: Option<&'a str>,
    pub side: Side,
    pub size: Decimal,
    pub price: Decimal,
    pub avg_fill_price: Option<Decimal>,
    pub filled_size: Decimal,
    pub remaining_size: Decimal,
    pub r#type: OrderType,
    pub status: OrderStatus,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub liquidation: bool,
    pub created_at: FtxDateTime,
}

impl<'a> TryFrom<OrderPartial<'a>> for Order<'a> {
    type Error = serde_json::Error;

    fn try_from(val: OrderPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: val.id.deserialize()?,
            client_id: val.client_id,
            market: val.market,
            future: val.future,
            side: val.side.deserialize()?,
            size: val.size.deserialize()?,
            price: val.price.deserialize()?,
            avg_fill_price: val.avg_fill_price.deserialize()?,
            filled_size: val.filled_size.deserialize()?,
            remaining_size: val.remaining_size.deserialize()?,
            r#type: val.r#type.deserialize()?,
            status: val.status.deserialize()?,
            reduce_only: val.reduce_only.deserialize()?,
            ioc: val.ioc.deserialize()?,
            post_only: val.post_only.deserialize()?,
            liquidation: val.liquidation.deserialize()?,
            created_at: val.created_at.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OrderPartial<'a> {
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    pub client_id: Option<&'a str>,
    pub market: &'a str,
    pub future: Option<&'a str>,
    #[serde(borrow)]
    pub side: Json<'a, Side>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub price: Json<'a, Decimal>,
    #[serde(borrow)]
    pub avg_fill_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub filled_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub remaining_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub r#type: Json<'a, OrderType>,
    #[serde(borrow)]
    pub status: Json<'a, OrderStatus>,
    #[serde(borrow)]
    pub reduce_only: Json<'a, bool>,
    #[serde(borrow)]
    pub ioc: Json<'a, bool>,
    #[serde(borrow)]
    pub post_only: Json<'a, bool>,
    #[serde(borrow)]
    pub liquidation: Json<'a, bool>,
    #[serde(borrow)]
    pub created_at: Json<'a, FtxDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OrderPlaced<'a> {
    pub id: u64,
    pub client_id: Option<&'a str>,
    pub market: &'a str,
    pub future: Option<&'a str>,
    pub side: Side,
    pub size: Decimal,
    pub price: Decimal,
    pub avg_fill_price: Option<Decimal>,
    pub filled_size: Decimal,
    pub remaining_size: Decimal,
    pub r#type: OrderType,
    pub status: OrderStatus,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub liquidation: Option<bool>,
    pub created_at: FtxDateTime,
}

impl<'a> TryFrom<OrderPlacedPartial<'a>> for OrderPlaced<'a> {
    type Error = serde_json::Error;

    fn try_from(val: OrderPlacedPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: val.id.deserialize()?,
            client_id: val.client_id,
            market: val.market,
            future: val.future,
            side: val.side.deserialize()?,
            size: val.size.deserialize()?,
            price: val.price.deserialize()?,
            avg_fill_price: val.avg_fill_price.deserialize()?,
            filled_size: val.filled_size.deserialize()?,
            remaining_size: val.remaining_size.deserialize()?,
            r#type: val.r#type.deserialize()?,
            status: val.status.deserialize()?,
            reduce_only: val.reduce_only.deserialize()?,
            ioc: val.ioc.deserialize()?,
            post_only: val.post_only.deserialize()?,
            liquidation: val.liquidation.deserialize()?,
            created_at: val.created_at.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct OrderPlacedPartial<'a> {
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    pub client_id: Option<&'a str>,
    pub market: &'a str,
    pub future: Option<&'a str>,
    #[serde(borrow)]
    pub side: Json<'a, Side>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub price: Json<'a, Decimal>,
    #[serde(borrow)]
    pub avg_fill_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub filled_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub remaining_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub r#type: Json<'a, OrderType>,
    #[serde(borrow)]
    pub status: Json<'a, OrderStatus>,
    #[serde(borrow)]
    pub reduce_only: Json<'a, bool>,
    #[serde(borrow)]
    pub ioc: Json<'a, bool>,
    #[serde(borrow)]
    pub post_only: Json<'a, bool>,
    #[serde(borrow)]
    pub liquidation: OptJson<'a, bool>,
    #[serde(borrow)]
    pub created_at: Json<'a, FtxDateTime>,
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::Response;

    use super::*;

    #[test]
    fn get_open_orders() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "createdAt": "2019-03-05T09:56:55.728933+00:00",
      "filledSize": 10,
      "future": "XRP-PERP",
      "id": 9596912,
      "market": "XRP-PERP",
      "price": 0.306525,
      "avgFillPrice": 0.306526,
      "remainingSize": 31421,
      "side": "sell",
      "size": 31431,
      "status": "open",
      "type": "limit",
      "reduceOnly": false,
      "ioc": false,
      "postOnly": false,
      "liquidation": false,
      "clientId": null
    }
  ]
}
"#;
        let response = GetOpenOrdersResponse(json.as_bytes().into());

        let from_partial: Vec<Order> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Order::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn get_order_history() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "avgFillPrice": 10135.25,
      "clientId": null,
      "createdAt": "2019-06-27T15:24:03.101197+00:00",
      "filledSize": 0.001,
      "future": "BTC-PERP",
      "id": 257132591,
      "ioc": false,
      "market": "BTC-PERP",
      "postOnly": false,
      "liquidation": false,
      "price": 10135.25,
      "reduceOnly": false,
      "remainingSize": 0.0,
      "side": "buy",
      "size": 0.001,
      "status": "closed",
      "type": "limit"
    }
  ],
  "hasMoreData": false
}
"#;
        let response = GetOpenOrdersResponse(json.as_bytes().into());

        let from_partial: Vec<Order> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Order::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn place_order() {
        let json = r#"
{
  "success": true,
  "result": {
    "createdAt": "2019-03-05T09:56:55.728933+00:00",
    "filledSize": 0,
    "future": "XRP-PERP",
    "id": 9596912,
    "market": "XRP-PERP",
    "price": 0.306525,
    "avgFillPrice": null,
    "remainingSize": 31431,
    "side": "sell",
    "size": 31431,
    "status": "open",
    "type": "limit",
    "reduceOnly": false,
    "ioc": false,
    "postOnly": false,
    "liquidation": false,
    "clientId": null
  }
}
"#;
        let response = PlaceOrderResponse(json.as_bytes().into());

        let from_partial: OrderPlaced<'_> =
            response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn edit_order() {
        let json = r#"
{
  "success": true,
  "result": {
    "createdAt": "2019-03-05T11:56:55.728933+00:00",
    "filledSize": 0,
    "future": "XRP-PERP",
    "id": 9596932,
    "market": "XRP-PERP",
    "price": 0.326525,
    "avgFillPrice": null,
    "remainingSize": 31431,
    "side": "sell",
    "size": 31431,
    "status": "open",
    "type": "limit",
    "reduceOnly": false,
    "ioc": false,
    "postOnly": false,
    "liquidation": false,
    "clientId": null
  }
}
"#;
        let response = EditOrderResponse(json.as_bytes().into());

        let from_partial: OrderPlaced<'_> =
            response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn get_order_status() {
        let json = r#"
{
  "success": true,
  "result": {
    "createdAt": "2019-03-05T09:56:55.728933+00:00",
    "filledSize": 10,
    "future": "XRP-PERP",
    "id": 9596912,
    "market": "XRP-PERP",
    "price": 0.306525,
    "avgFillPrice": 0.306526,
    "remainingSize": 31421,
    "side": "sell",
    "size": 31431,
    "status": "open",
    "type": "limit",
    "reduceOnly": false,
    "ioc": false,
    "postOnly": false,
    "liquidation": false,
    "clientId": "your_client_order_id"
  }
}
"#;
        let response = GetOrderStatusResponse(json.as_bytes().into());

        let from_partial: Order<'_> = response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn cancel_order() {
        let json = r#"
{
  "success": true,
  "result": "Order queued for cancelation"
}
"#;
        let response = CancelOrderResponse(json.as_bytes().into());

        let from_partial: CancelAckMsg<'_> = response.deserialize_partial().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn cancel_all_orders() {
        let json = r#"
{
  "success": true,
  "result": "Orders queued for cancelation"
}
"#;
        let response = CancelAllOrdersResponse(json.as_bytes().into());

        let from_partial: CancelAckMsg<'_> = response.deserialize_partial().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }
}
