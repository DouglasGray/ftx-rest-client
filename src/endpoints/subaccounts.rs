use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, convert::TryFrom};

use crate::{data::FtxDateTime, private::Sealed, Json, Request};

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

response!(
    GetSubaccountsResponse,
    Vec<Subaccount<'a>>,
    Vec<SubaccountPartial<'a>>
);

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

response!(
    CreateSubaccountResponse,
    Subaccount<'a>,
    SubaccountPartial<'a>
);

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

response!(ChangeSubaccountNameResponse, (), ());

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

response!(DeleteSubaccountResponse, (), ());

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

response!(
    GetSubaccountBalancesResponse,
    Vec<SubaccountBalance<'a>>,
    Vec<SubaccountBalancePartial<'a>>
);

/// Transfer between subaccounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenSubaccounts<'a> {
    pub coin: &'a str,
    pub size: Decimal,
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

response!(
    TransferBetweenSubaccountsResponse,
    TransferDetails<'a>,
    TransferDetailsPartial<'a>
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Subaccount<'a> {
    pub nickname: &'a str,
    pub deletable: bool,
    pub editable: bool,
    pub special: bool,
    pub competition: bool,
}

impl<'a> TryFrom<SubaccountPartial<'a>> for Subaccount<'a> {
    type Error = serde_json::Error;

    fn try_from(val: SubaccountPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            nickname: val.nickname,
            deletable: val.deletable.deserialize()?,
            editable: val.editable.deserialize()?,
            special: val.special.deserialize()?,
            competition: val.competition.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubaccountPartial<'a> {
    pub nickname: &'a str,
    #[serde(borrow)]
    pub deletable: Json<'a, bool>,
    #[serde(borrow)]
    pub editable: Json<'a, bool>,
    #[serde(borrow)]
    pub special: Json<'a, bool>,
    #[serde(borrow)]
    pub competition: Json<'a, bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubaccountBalance<'a> {
    pub coin: &'a str,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
    pub available_for_withdrawal: Decimal,
    pub usd_value: Decimal,
}

impl<'a> TryFrom<SubaccountBalancePartial<'a>> for SubaccountBalance<'a> {
    type Error = serde_json::Error;

    fn try_from(val: SubaccountBalancePartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            coin: val.coin,
            free: val.free.deserialize()?,
            total: val.total.deserialize()?,
            spot_borrow: val.spot_borrow.deserialize()?,
            available_without_borrow: val.available_without_borrow.deserialize()?,
            available_for_withdrawal: val.available_for_withdrawal.deserialize()?,
            usd_value: val.usd_value.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubaccountBalancePartial<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub free: Json<'a, Decimal>,
    #[serde(borrow)]
    pub total: Json<'a, Decimal>,
    #[serde(borrow)]
    pub spot_borrow: Json<'a, Decimal>,
    #[serde(borrow)]
    pub available_without_borrow: Json<'a, Decimal>,
    #[serde(borrow)]
    pub available_for_withdrawal: Json<'a, Decimal>,
    #[serde(borrow)]
    pub usd_value: Json<'a, Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TransferDetails<'a> {
    pub id: u64,
    pub coin: &'a str,
    pub size: Decimal,
    pub time: FtxDateTime,
    pub notes: &'a str,
    pub status: TransferStatus,
}

impl<'a> TryFrom<TransferDetailsPartial<'a>> for TransferDetails<'a> {
    type Error = serde_json::Error;

    fn try_from(val: TransferDetailsPartial<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: val.id.deserialize()?,
            coin: val.coin,
            size: val.size.deserialize()?,
            time: val.time.deserialize()?,
            notes: val.notes,
            status: val.status.deserialize()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TransferDetailsPartial<'a> {
    #[serde(borrow)]
    pub id: Json<'a, u64>,
    pub coin: &'a str,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub time: Json<'a, FtxDateTime>,
    pub notes: &'a str,
    #[serde(borrow)]
    pub status: Json<'a, TransferStatus>,
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

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
        let response = GetSubaccountsResponse(json.as_bytes().into());

        let from_partial: Vec<Subaccount<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| Subaccount::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = CreateSubaccountResponse(json.as_bytes().into());

        let from_partial: Subaccount<'_> =
            response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }

    #[test]
    fn change_subaccount_name() {
        let json = r#"
{
  "success": true,
  "result": null
}
"#;
        let _: () = ChangeSubaccountNameResponse(json.as_bytes().into())
            .deserialize_partial()
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
        let response = DeleteSubaccountResponse(json.as_bytes().into());

        let from_partial: () = response.deserialize_partial().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = GetSubaccountBalancesResponse(json.as_bytes().into());

        let from_partial: Vec<SubaccountBalance<'_>> = response
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| SubaccountBalance::try_from(p).unwrap())
            .collect();

        assert_eq!(response.deserialize().unwrap(), from_partial);
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
        let response = TransferBetweenSubaccountsResponse(json.as_bytes().into());

        let from_partial: TransferDetails<'_> =
            response.deserialize_partial().unwrap().try_into().unwrap();

        assert_eq!(response.deserialize().unwrap(), from_partial);
    }
}
