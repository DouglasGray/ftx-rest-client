use std::convert::TryFrom;

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{FtxDateTime, UnixTimestamp},
    private::Sealed,
    Json, OptJson, Request,
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

response!(
    GetBorrowRatesResponse,
    Vec<BorrowRate<'a>>,
    Vec<BorrowRatePartial<'a>>
);

/// Retrieve the latest lending rates for all spot margin enabled
/// coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetLendingRates;

impl Sealed for GetLendingRates {}

impl Request<false> for GetLendingRates {
    const PATH: &'static str = "/spot_margin/lending_rates";

    const METHOD: Method = Method::GET;

    type Response = GetLendingRatesResponse;
}

pub struct GetLendingRatesResponse(Bytes);

response!(
    GetLendingRatesResponse,
    Vec<LendingRate<'a>>,
    Vec<LendingRatePartial<'a>>
);

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

response!(
    GetDailyBorrowedAmountsResponse,
    Vec<BorrowAmount<'a>>,
    Vec<BorrowAmountPartial<'a>>
);

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

response!(
    GetBorrowMarketsResponse,
    Option<Vec<BorrowMarket<'a>>>,
    Option<Vec<BorrowMarketPartial<'a>>>
);

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

response!(
    GetBorrowHistoryResponse,
    Vec<BorrowPayment<'a>>,
    Vec<BorrowPaymentPartial<'a>>
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowRate<'a> {
    pub coin: &'a str,
    pub estimate: Decimal,
    pub previous: Decimal,
    pub average_24hr: Option<Decimal>,
}

impl<'a> TryFrom<BorrowRatePartial<'a>> for BorrowRate<'a> {
    type Error = serde_json::Error;

    fn try_from(val: BorrowRatePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            estimate: val.estimate.deserialize()?,
            previous: val.previous.deserialize()?,
            average_24hr: val.average_24hr.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowRatePartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub estimate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub previous: Json<'a, Decimal>,
    #[serde(borrow)]
    pub average_24hr: OptJson<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LendingRate<'a> {
    pub coin: &'a str,
    pub estimate: Decimal,
    pub previous: Decimal,
    pub average_24hr: Option<Decimal>,
}

impl<'a> TryFrom<LendingRatePartial<'a>> for LendingRate<'a> {
    type Error = serde_json::Error;

    fn try_from(val: LendingRatePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            estimate: val.estimate.deserialize()?,
            previous: val.previous.deserialize()?,
            average_24hr: val.average_24hr.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LendingRatePartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub estimate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub previous: Json<'a, Decimal>,
    #[serde(borrow)]
    pub average_24hr: OptJson<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowAmount<'a> {
    pub coin: &'a str,
    pub size: Decimal,
}

impl<'a> TryFrom<BorrowAmountPartial<'a>> for BorrowAmount<'a> {
    type Error = serde_json::Error;

    fn try_from(val: BorrowAmountPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            size: val.size.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowAmountPartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowMarket<'a> {
    pub coin: &'a str,
    pub borrowed: Decimal,
    pub free: Decimal,
    pub estimated_rate: Decimal,
    pub previous_rate: Decimal,
}

impl<'a> TryFrom<BorrowMarketPartial<'a>> for BorrowMarket<'a> {
    type Error = serde_json::Error;

    fn try_from(val: BorrowMarketPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            borrowed: val.borrowed.deserialize()?,
            free: val.free.deserialize()?,
            estimated_rate: val.estimated_rate.deserialize()?,
            previous_rate: val.previous_rate.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowMarketPartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub borrowed: Json<'a, Decimal>,
    #[serde(borrow)]
    pub free: Json<'a, Decimal>,
    #[serde(borrow)]
    pub estimated_rate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub previous_rate: Json<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowPayment<'a> {
    pub coin: &'a str,
    pub cost: Decimal,
    pub fee_usd: Decimal,
    pub rate: Decimal,
    pub size: Decimal,
    pub time: FtxDateTime,
}

impl<'a> TryFrom<BorrowPaymentPartial<'a>> for BorrowPayment<'a> {
    type Error = serde_json::Error;

    fn try_from(val: BorrowPaymentPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            cost: val.cost.deserialize()?,
            fee_usd: val.fee_usd.deserialize()?,
            rate: val.rate.deserialize()?,
            size: val.size.deserialize()?,
            time: val.time.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BorrowPaymentPartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub cost: Json<'a, Decimal>,
    #[serde(borrow)]
    pub fee_usd: Json<'a, Decimal>,
    #[serde(borrow)]
    pub rate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

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
      "previous": 1.44e-06,
      "average24hr": 1.44e-06
    }
  ]
}
"#;
        let response = GetBorrowRatesResponse(json.as_bytes().into());

        let from_partial: Vec<BorrowRate> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| BorrowRate::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn get_lending_rates() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "BTC",
      "estimate": 1.45e-06,
      "previous": 1.44e-06,
      "average24hr": 1.44e-06
    }
  ]
}
"#;
        let response = GetLendingRatesResponse(json.as_bytes().into());

        let from_partial: Vec<LendingRate> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| LendingRate::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = GetDailyBorrowedAmountsResponse(json.as_bytes().into());

        let from_partial: Vec<BorrowAmount> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| BorrowAmount::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = GetBorrowMarketsResponse(json.as_bytes().into());

        let from_partial: Vec<BorrowMarket<'_>> = response
            .deserialize_partial()
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|p| BorrowMarket::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap().unwrap(), from_partial);
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
        let response = GetBorrowHistoryResponse(json.as_bytes().into());

        let from_partial: Vec<BorrowPayment<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| BorrowPayment::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }
}
