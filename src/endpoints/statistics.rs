use std::convert::TryFrom;

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{private::Sealed, Json, OptJson, QueryParams, Request};

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

response!(
    GetLatencyStatisticsResponse,
    Vec<LatencyStats>,
    Vec<LatencyStatsPartial<'a>>
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LatencyStats {
    pub bursty: bool,
    pub proxied: bool,
    pub p50: Option<Decimal>,
    pub request_count: u64,
    pub success_count: u64,
    pub success_p50: Option<Decimal>,
}

impl<'a> TryFrom<LatencyStatsPartial<'a>> for LatencyStats {
    type Error = serde_json::Error;

    fn try_from(val: LatencyStatsPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            bursty: val.bursty.deserialize()?,
            proxied: val.proxied.deserialize()?,
            p50: val.p50.deserialize()?,
            request_count: val.request_count.deserialize()?,
            success_count: val.success_count.deserialize()?,
            success_p50: val.success_p50.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LatencyStatsPartial<'a> {
    #[serde(borrow)]
    pub bursty: Json<'a, bool>,
    #[serde(borrow)]
    pub proxied: Json<'a, bool>,
    #[serde(borrow)]
    pub p50: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub request_count: Json<'a, u64>,
    #[serde(borrow)]
    pub success_count: Json<'a, u64>,
    #[serde(borrow)]
    pub success_p50: OptJson<'a, Decimal>,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

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
      "proxied": true,
      "p50": 0.059,
      "requestCount": 43,
      "successCount": 0,
      "successP50": null
    },
    {
      "bursty": false,
      "proxied": true,
      "p50": 0.047,
      "requestCount": 27,
      "successCount": 27,
      "successP50": 0.047
    }
  ]
}
"#;
        let response = GetLatencyStatisticsResponse(json.as_bytes().into());

        let from_partial: Vec<LatencyStats> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| LatencyStats::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }
}
