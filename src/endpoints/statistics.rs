use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{data::NonNegativeDecimal, private::Sealed, QueryParams, Request};

use super::macros::response;

/// Retrieve the latest borrow rates for all spot margin enabled
/// coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetLatencyStatistics<'a> {
    pub days: Option<u32>,
    pub subaccount_nickname: Option<&'a str>,
}

impl<'a> Sealed for GetLatencyStatistics<'a> {}

impl<'a> Request<true> for GetLatencyStatistics<'a> {
    const PATH: &'static str = "/stats/latency_stats";

    const METHOD: Method = Method::GET;

    type Response = GetLatencyStatisticsResponse;

    fn query_params(&self) -> Option<QueryParams> {
        if self.days.is_none() && self.subaccount_nickname.is_none() {
            return None;
        }

        let mut params = Vec::with_capacity(2);

        if let Some(days) = self.days {
            params.push(("days", days.to_string()))
        }
        if let Some(name) = self.subaccount_nickname {
            params.push(("subaccount_nickname", name.into()))
        }

        Some(params)
    }
}

pub struct GetLatencyStatisticsResponse(Bytes);

response!(GetLatencyStatisticsResponse, Vec<LatencyStats>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LatencyStats {
    pub bursty: bool,
    pub p50: NonNegativeDecimal,
    pub request_count: u64,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_latency_statistics() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "bursty": true,
      "p50": 0.059,
      "requestCount": 43
    },
    {
      "bursty": false,
      "p50": 0.047,
      "requestCount": 27
    }
  ]
}
"#;
        GetLatencyStatisticsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
