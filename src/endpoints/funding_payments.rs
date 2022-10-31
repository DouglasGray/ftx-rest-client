use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{FtxDateTime, UnixTimestamp},
    private::Sealed,
    Json, Request,
};

use super::macros::response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetFundingPayments<'a> {
    pub future: Option<&'a str>,
    pub start_time: Option<UnixTimestamp>,
    pub end_time: Option<UnixTimestamp>,
}

impl<'a> Sealed for GetFundingPayments<'a> {}

impl<'a> Request<true> for GetFundingPayments<'a> {
    const PATH: &'static str = "/funding_payments";

    const METHOD: Method = Method::GET;

    type Response = GetFundingPaymentsResponse;

    fn query_params(&self) -> Option<crate::QueryParams> {
        if self.future.is_none() && self.start_time.is_none() && self.end_time.is_none() {
            return None;
        }

        let mut params = Vec::with_capacity(3);

        if let Some(future) = self.future {
            params.push(("future", future.into()));
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

pub struct GetFundingPaymentsResponse(Bytes);

response!(GetFundingPaymentsResponse, Vec<FundingPayment<'a>>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundingPayment<'a> {
    pub future: &'a str,
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    #[serde(borrow)]
    pub payment: Json<'a, Decimal>,
    #[serde(borrow)]
    pub rate: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
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
    pub struct ParsedFundingPayment<'a> {
        pub future: &'a str,
        pub id: u64,
        pub payment: Decimal,
        pub rate: Decimal,
        pub time: FtxDateTime,
    }

    impl<'a> TryFrom<FundingPayment<'a>> for ParsedFundingPayment<'a> {
        type Error = serde_json::Error;

        fn try_from(val: FundingPayment<'a>) -> Result<Self, Self::Error> {
            Ok(ParsedFundingPayment {
                future: val.future,
                id: val.id.deserialize()?,
                payment: val.payment.deserialize()?,
                rate: val.rate.deserialize()?,
                time: val.time.deserialize()?,
            })
        }
    }

    #[test]
    fn get_funding_payments() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "future": "ETH-PERP",
      "id": 33830,
      "payment": 0.0441342,
      "time": "2019-05-15T18:00:00+00:00",
      "rate": 0.0001
    }
  ]
}
"#;
        let _: Vec<ParsedFundingPayment<'_>> = GetFundingPaymentsResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| ParsedFundingPayment::try_from(p).unwrap())
            .collect();
    }
}
