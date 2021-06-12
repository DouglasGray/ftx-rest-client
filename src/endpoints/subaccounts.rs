use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    data::{DateTimeStr, NonNegativeDecimal, Size},
    private::Sealed,
    Request,
};

use super::macros::response;

macro_rules! get_subaccount_balances_path {
    () => {
        "/subaccounts/{nickname}/balances"
    };
}

/// Status of an inter-account transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferStatus {
    #[serde(rename = "complete")]
    Complete,
}

/// Retrieve info on all subaccounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetSubaccounts;

impl Sealed for GetSubaccounts {}

impl Request<true> for GetSubaccounts {
    const PATH: &'static str = "/subaccounts";

    const METHOD: Method = Method::GET;

    type Response = GetSubaccountsResponse;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSubaccountsResponse(Bytes);

response!(GetSubaccountsResponse, Vec<Subaccount<'de>>);

/// Create a subaccount.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubaccount<'a> {
    pub nickname: &'a str,
}

impl<'a> Sealed for CreateSubaccount<'a> {}

impl<'a> Request<true> for CreateSubaccount<'a> {
    const PATH: &'static str = "/subaccounts";

    const METHOD: Method = Method::POST;

    type Response = CreateSubaccountResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct CreateSubaccountResponse(Bytes);

response!(CreateSubaccountResponse, Subaccount<'de>);

/// Change a subaccount name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubaccountName<'a> {
    pub nickname: &'a str,
    pub new_nickname: &'a str,
}

impl<'a> Sealed for ChangeSubaccountName<'a> {}

impl<'a> Request<true> for ChangeSubaccountName<'a> {
    const PATH: &'static str = "/subaccounts/update_name";

    const METHOD: Method = Method::POST;

    type Response = ChangeSubaccountNameResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct ChangeSubaccountNameResponse(Bytes);

response!(ChangeSubaccountNameResponse, ());

/// Delete subaccount
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubaccount<'a> {
    pub nickname: &'a str,
}

impl<'a> Sealed for DeleteSubaccount<'a> {}

impl<'a> Request<true> for DeleteSubaccount<'a> {
    const PATH: &'static str = "/subaccounts";

    const METHOD: Method = Method::DELETE;

    type Response = DeleteSubaccountResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct DeleteSubaccountResponse(Bytes);

response!(DeleteSubaccountResponse, ());

/// Retrieve a subaccount's balances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetSubaccountBalances<'a> {
    pub nickname: &'a str,
}

impl<'a> Sealed for GetSubaccountBalances<'a> {}

impl<'a> Request<true> for GetSubaccountBalances<'a> {
    const PATH: &'static str = get_subaccount_balances_path!();

    const METHOD: Method = Method::GET;

    type Response = GetSubaccountBalancesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            get_subaccount_balances_path!(),
            nickname = self.nickname
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSubaccountBalancesResponse(Bytes);

response!(GetSubaccountBalancesResponse, Vec<SubaccountBalance<'de>>);

/// Transfer between subaccounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenSubaccounts<'a> {
    pub coin: &'a str,
    pub size: Size,
    pub source: Option<&'a str>,
    pub destination: Option<&'a str>,
}

impl<'a> Sealed for TransferBetweenSubaccounts<'a> {}

impl<'a> Request<true> for TransferBetweenSubaccounts<'a> {
    const PATH: &'static str = "/subaccounts/transfer";

    const METHOD: Method = Method::POST;

    type Response = TransferBetweenSubaccountsResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct TransferBetweenSubaccountsResponse(Bytes);

response!(TransferBetweenSubaccountsResponse, TransferDetails<'de>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Subaccount<'a> {
    #[serde(borrow)]
    pub nickname: &'a str,
    pub deletable: bool,
    pub editable: bool,
    pub special: bool,
    pub competition: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubaccountBalance<'a> {
    #[serde(borrow)]
    pub coin: &'a str,
    pub free: NonNegativeDecimal,
    pub total: NonNegativeDecimal,
    pub spot_borrow: NonNegativeDecimal,
    pub available_without_borrow: NonNegativeDecimal,
    pub available_for_withdrawal: NonNegativeDecimal,
    pub usd_value: NonNegativeDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TransferDetails<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub coin: &'a str,
    pub size: Size,
    #[serde(borrow)]
    pub time: DateTimeStr<'a>,
    #[serde(borrow)]
    pub notes: &'a str,
    pub status: TransferStatus,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_subaccounts() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "nickname": "sub1",
      "deletable": true,
      "editable": true,
      "competition": true,
      "special": false
    }
  ]
}
"#;
        GetSubaccountsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn create_subaccount() {
        let json = r#"
{
  "success": true,
  "result": {
    "nickname": "sub2",
    "deletable": true,
    "editable": true,
    "special": false,
    "competition": false
  }
}
"#;
        CreateSubaccountResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn change_subaccount_name() {
        let json = r#"
{
  "success": true,
  "result": null
}
"#;
        ChangeSubaccountNameResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn delete_subaccount() {
        let json = r#"
{
  "success": true,
  "result": null
}
"#;
        DeleteSubaccountResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_subaccount_balances() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "USDT",
      "free": 4321.2,
      "total": 4340.2,
      "spotBorrow": 0,
      "availableWithoutBorrow": 2320.2,
      "availableForWithdrawal": 2320.2,
      "usdValue": 4320.1
    }
  ]
}
"#;

        GetSubaccountBalancesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn transfer_between_subaccounts() {
        let json = r#"
{
  "success": true,
  "result": {
    "id": 316450,
    "coin": "XRP",
    "size": 10000,
    "time": "2019-03-05T09:56:55.728933+00:00",
    "notes": "",
    "status": "complete"
  }
}
"#;

        TransferBetweenSubaccountsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
