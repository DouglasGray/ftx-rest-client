use std::collections::HashMap;

use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    data::{AccountName, NonNegativeDecimal, PositiveDecimal},
    private::Sealed,
    Request,
};

use super::macros::response;

/// Retrieve info on all coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetCoins;

impl Sealed for GetCoins {}

impl Request<true> for GetCoins {
    const PATH: &'static str = "/wallet/coins";

    const METHOD: Method = Method::GET;

    type Response = GetCoinsResponse;
}

pub struct GetCoinsResponse(Bytes);

response!(GetCoinsResponse, Vec<Coin<'de>>);

/// Retrieve coin balances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetBalances;

impl Sealed for GetBalances {}

impl Request<true> for GetBalances {
    const PATH: &'static str = "/wallet/balances";

    const METHOD: Method = Method::GET;

    type Response = GetBalancesResponse;
}

pub struct GetBalancesResponse(Bytes);

response!(GetBalancesResponse, Vec<Balance<'de>>);

/// Retrieve coin balances for all account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetAllBalances;

impl Sealed for GetAllBalances {}

impl Request<true> for GetAllBalances {
    const PATH: &'static str = "/wallet/all_balances";

    const METHOD: Method = Method::GET;

    type Response = GetAllBalancesResponse;
}

pub struct GetAllBalancesResponse(Bytes);

response!(
    GetAllBalancesResponse,
    HashMap<AccountName<'de>, Vec<Balance<'de>>>
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Coin<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub fiat: bool,
    pub is_token: bool,
    pub is_etf: bool,
    pub tokenized_equity: Option<bool>,
    pub spot_margin: bool,
    pub collateral: bool,
    pub collateral_weight: PositiveDecimal,
    pub usd_fungible: bool,
    pub can_convert: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub erc20_contract: Option<&'a str>,
    pub trc20_contract: Option<&'a str>,
    pub bep2_asset: Option<&'a str>,
    pub spl_mint: Option<&'a str>,
    pub methods: Vec<&'a str>,
    pub has_tag: bool,
    pub credit_to: Option<&'a str>,
    pub hidden: bool,
    pub image_url: Option<&'a str>,
    pub nft_quote_currency_eligible: bool,
    pub imf_weight: PositiveDecimal,
    // Keep this as a float since the values returned occasionally
    // have a crazy scale and can't be parsed into a `Decimal`.
    pub index_price: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Balance<'a> {
    pub coin: &'a str,
    pub free: NonNegativeDecimal,
    pub spot_borrow: NonNegativeDecimal,
    pub total: NonNegativeDecimal,
    pub usd_value: NonNegativeDecimal,
    pub available_without_borrow: NonNegativeDecimal,
    pub available_for_withdrawal: NonNegativeDecimal,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_coins() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "bep2Asset": null,
      "canConvert": true,
      "canDeposit": false,
      "canWithdraw": false,
      "collateral": true,
      "collateralWeight": 1,
      "creditTo": null,
      "erc20Contract": null,
      "fiat": true,
      "hasTag": false,
      "hidden": false,
      "id": "USD",
      "imageUrl": null,
      "indexPrice": 1,
      "isEtf": false,
      "isToken": false,
      "methods": [],
      "name": "USD",
      "nftQuoteCurrencyEligible": true,
      "splMint": null,
      "spotMargin": true,
      "trc20Contract": null,
      "usdFungible": true,
      "imfWeight": 1.0
    }
  ]
}
"#;
        GetCoinsResponse(json.as_bytes().into()).to_data().unwrap();
    }

    #[test]
    fn get_balances() {
        let json = r#"
{
  "success": true,
  "result": [
    {
      "coin": "USDTBEAR",
      "free": 2320.2,
      "spotBorrow": 0.0,
      "total": 2340.2,
      "usdValue": 2340.2,
      "availableWithoutBorrow": 2320.2,
      "availableForWithdrawal": 2320.2
    }
  ]
}
"#;
        GetBalancesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_all_balances() {
        let json = r#"
{
  "success": true,
  "result": {
    "main": [
      {
        "coin": "USDTBEAR",
        "free": 2320.2,
        "spotBorrow": 0.0,
        "total": 2340.2,
        "usdValue": 2340.2,
        "availableWithoutBorrow": 2320.2,
        "availableForWithdrawal": 2320.2
      },
      {
        "coin": "BTC",
        "free": 2.0,
        "spotBorrow": 0.0,
        "total": 3.2,
        "usdValue": 23456.7,
        "availableWithoutBorrow": 2.0,
        "availableForWithdrawal": 2.0
      }
    ],
    "Battle Royale": [
      {
        "coin": "USD",
        "free": 2000.0,
        "spotBorrow": 0.0,
        "total": 2200.0,
        "usdValue": 2200.0,
        "availableWithoutBorrow": 2000.0,
        "availableForWithdrawal": 2000.0
      }
    ]
  }
}
"#;
        GetAllBalancesResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
