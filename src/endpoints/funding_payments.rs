use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    data::{DateTimeStr, UnixTimestamp},
    private::Sealed,
    Request,
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

response!(GetFundingPaymentsResponse, Vec<FundingPayment<'de>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundingPayment<'a> {
    pub future: &'a str,
    pub id: u64,
    pub payment: Decimal,
    pub rate: Decimal,
    pub time: DateTimeStr<'a>,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

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
        GetFundingPaymentsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
