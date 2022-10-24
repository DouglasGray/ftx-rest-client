use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    data::{DateTimeStr, NonNegativeDecimal, PositiveDecimal, UnixTimestamp},
    private::Sealed,
    Request,
};

use super::macros::response;

/// Retrieve the latest borrow rates for all spot margin enabled
/// coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetBorrowRates;

impl Sealed for GetBorrowRates {}

impl Request<true> for GetBorrowRates {
    const PATH: &'static str = "/spot_margin/borrow_rates";

    const METHOD: Method = Method::GET;

    type Response = GetBorrowRatesResponse;
}

pub struct GetBorrowRatesResponse(Bytes);

response!(GetBorrowRatesResponse, Vec<BorrowRate<'a>>);

/// Retrieve the total daily borrowed amounts for all spot margin
/// enabled coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetDailyBorrowedAmounts;

impl Sealed for GetDailyBorrowedAmounts {}

impl Request<true> for GetDailyBorrowedAmounts {
    const PATH: &'static str = "/spot_margin/borrow_summary";

    const METHOD: Method = Method::GET;

    type Response = GetDailyBorrowedAmountsResponse;
}

pub struct GetDailyBorrowedAmountsResponse(Bytes);

response!(GetDailyBorrowedAmountsResponse, Vec<BorrowAmount<'a>>);

/// Retrieve information on borrow rates for the provided spot market,
/// i.e. what the rates are for the quote and base currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetBorrowForMarket<'a> {
    pub spot_market: &'a str,
}

impl<'a> Sealed for GetBorrowForMarket<'a> {}

impl<'a> Request<true> for GetBorrowForMarket<'a> {
    const PATH: &'static str = "/spot_margin/market_info";

    const METHOD: Method = Method::GET;

    type Response = GetBorrowMarketsResponse;

    fn query_params(&self) -> Option<crate::QueryParams> {
        Some(vec![("market", self.spot_market.into())])
    }
}

pub struct GetBorrowMarketsResponse(Bytes);

response!(GetBorrowMarketsResponse, Option<Vec<BorrowMarket<'a>>>);

/// Retrieve an account's borrow history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetBorrowHistory {
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl Sealed for GetBorrowHistory {}

impl Request<true> for GetBorrowHistory {
    const PATH: &'static str = "/spot_margin/borrow_history";

    const METHOD: Method = Method::GET;

    type Response = GetBorrowHistoryResponse;

    fn query_params(&self) -> Option<crate::QueryParams> {
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

pub struct GetBorrowHistoryResponse(Bytes);

response!(GetBorrowHistoryResponse, Vec<BorrowPayment<'a>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowRate<'a> {
    pub coin: &'a str,
    pub estimate: PositiveDecimal,
    pub previous: PositiveDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowAmount<'a> {
    pub coin: &'a str,
    pub size: NonNegativeDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowMarket<'a> {
    pub coin: &'a str,
    pub borrowed: NonNegativeDecimal,
    pub free: NonNegativeDecimal,
    pub estimated_rate: PositiveDecimal,
    pub previous_rate: PositiveDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowPayment<'a> {
    pub coin: &'a str,
    pub cost: NonNegativeDecimal,
    pub fee_usd: NonNegativeDecimal,
    pub rate: PositiveDecimal,
    pub size: PositiveDecimal,
    pub time: DateTimeStr<'a>,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_borrow_rates() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "BTC",
      "estimate": 1.45e-06,
      "previous": 1.44e-06
    }
  ]
}
"#;
        GetBorrowRatesResponse(json.as_bytes().into())
            .parse()
            .unwrap();
    }

    #[test]
    fn get_daily_borrowed_amounts() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "BTC",
      "size": 120.1
    }
  ]
}
"#;
        GetDailyBorrowedAmountsResponse(json.as_bytes().into())
            .parse()
            .unwrap();
    }

    #[test]
    fn get_borrow_for_market() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "BTC",
      "borrowed": 0.0,
      "free": 3.87278021,
      "estimatedRate": 1e-06,
      "previousRate": 1e-06
    },
    {
      "coin": "USD",
      "borrowed": 0.0,
      "free": 69966.22310497,
      "estimatedRate": 1.027e-05,
      "previousRate": 1.027e-05
    }
  ]
}
"#;
        GetBorrowMarketsResponse(json.as_bytes().into())
            .parse()
            .unwrap();
    }

    #[test]
    fn get_borrow_history() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "USD",
      "cost": 0.0075789748770483,
      "feeUsd": 0.0075789748770483,
      "rate": 0.0000292815,
      "size": 258.83151058,
      "time": "2021-05-13T08:00:00+00:00"
    }
  ]
}
"#;
        GetBorrowHistoryResponse(json.as_bytes().into())
            .parse()
            .unwrap();
    }
}
