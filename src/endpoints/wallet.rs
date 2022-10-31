use std::collections::HashMap;

use bytes::Bytes;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{data::AccountName, private::Sealed, Json, OptJson, Request};

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

response!(GetCoinsResponse, Vec<Coin<'a>>);

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

response!(GetBalancesResponse, Vec<Balance<'a>>);

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
    HashMap<AccountName<'a>, Vec<Balance<'a>>>
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Coin<'a> {
    pub id: &'a str,
    pub name: &'a str,
    #[serde(borrow)]
    pub fiat: Json<'a, bool>,
    #[serde(borrow)]
    pub is_token: Json<'a, bool>,
    #[serde(borrow)]
    pub is_etf: Json<'a, bool>,
    #[serde(borrow)]
    pub tokenized_equity: OptJson<'a, bool>,
    #[serde(borrow)]
    pub spot_margin: Json<'a, bool>,
    #[serde(borrow)]
    pub collateral: Json<'a, bool>,
    #[serde(borrow)]
    pub collateral_weight: Json<'a, Decimal>,
    #[serde(borrow)]
    pub usd_fungible: Json<'a, bool>,
    #[serde(borrow)]
    pub can_convert: Json<'a, bool>,
    #[serde(borrow)]
    pub can_deposit: Json<'a, bool>,
    #[serde(borrow)]
    pub can_withdraw: Json<'a, bool>,
    pub erc20_contract: Option<&'a str>,
    pub trc20_contract: Option<&'a str>,
    pub bep2_asset: Option<&'a str>,
    pub spl_mint: Option<&'a str>,
    #[serde(borrow)]
    pub methods: Vec<&'a str>,
    #[serde(borrow)]
    pub has_tag: Json<'a, bool>,
    pub credit_to: Option<&'a str>,
    #[serde(borrow)]
    pub hidden: Json<'a, bool>,
    pub image_url: Option<&'a str>,
    #[serde(borrow)]
    pub nft_quote_currency_eligible: Json<'a, bool>,
    #[serde(borrow)]
    pub imf_weight: Json<'a, Decimal>,
    // Keep this as a float since the values returned occasionally
    // have a crazy scale and can't be parsed into a `Json<'a, Decimal>`.
    #[serde(borrow)]
    index_price: Json<'a, f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Balance<'a> {
    pub coin: &'a str,
    #[serde(borrow)]
    pub free: Json<'a, Decimal>,
    #[serde(borrow)]
    pub spot_borrow: Json<'a, Decimal>,
    #[serde(borrow)]
    pub total: Json<'a, Decimal>,
    #[serde(borrow)]
    pub usd_value: Json<'a, Decimal>,
    #[serde(borrow)]
    pub available_without_borrow: Json<'a, Decimal>,
    #[serde(borrow)]
    pub available_for_withdrawal: Json<'a, Decimal>,
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::Response;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct ParsedCoin<'a> {
        pub id: &'a str,
        pub name: &'a str,
        pub fiat: bool,
        pub is_token: bool,
        pub is_etf: bool,
        pub tokenized_equity: Option<bool>,
        pub spot_margin: bool,
        pub collateral: bool,
        pub collateral_weight: Decimal,
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
        pub imf_weight: Decimal,
        // Keep this as a float since the values returned occasionally
        // have a crazy scale and can't be parsed into a `Decimal`.
        index_price: f64,
    }

    impl<'a> TryFrom<Coin<'a>> for ParsedCoin<'a> {
        type Error = serde_json::Error;

        fn try_from(val: Coin<'a>) -> Result<Self, Self::Error> {
            Ok(Self {
                id: val.id,
                name: val.name,
                fiat: val.fiat.deserialize()?,
                is_token: val.is_token.deserialize()?,
                is_etf: val.is_etf.deserialize()?,
                tokenized_equity: val.tokenized_equity.deserialize()?,
                spot_margin: val.spot_margin.deserialize()?,
                collateral: val.collateral.deserialize()?,
                collateral_weight: val.collateral_weight.deserialize()?,
                usd_fungible: val.usd_fungible.deserialize()?,
                can_convert: val.can_convert.deserialize()?,
                can_deposit: val.can_deposit.deserialize()?,
                can_withdraw: val.can_withdraw.deserialize()?,
                erc20_contract: val.erc20_contract,
                trc20_contract: val.trc20_contract,
                bep2_asset: val.bep2_asset,
                spl_mint: val.spl_mint,
                methods: val.methods,
                has_tag: val.has_tag.deserialize()?,
                credit_to: val.credit_to,
                hidden: val.hidden.deserialize()?,
                image_url: val.image_url,
                nft_quote_currency_eligible: val.nft_quote_currency_eligible.deserialize()?,
                imf_weight: val.imf_weight.deserialize()?,
                index_price: val.index_price.deserialize()?,
            })
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct ParsedBalance<'a> {
        pub coin: &'a str,
        pub free: Decimal,
        pub spot_borrow: Decimal,
        pub total: Decimal,
        pub usd_value: Decimal,
        pub available_without_borrow: Decimal,
        pub available_for_withdrawal: Decimal,
    }

    impl<'a> TryFrom<Balance<'a>> for ParsedBalance<'a> {
        type Error = serde_json::Error;

        fn try_from(val: Balance<'a>) -> Result<Self, Self::Error> {
            Ok(Self {
                coin: val.coin,
                free: val.free.deserialize()?,
                spot_borrow: val.spot_borrow.deserialize()?,
                total: val.total.deserialize()?,
                usd_value: val.usd_value.deserialize()?,
                available_without_borrow: val.available_without_borrow.deserialize()?,
                available_for_withdrawal: val.available_for_withdrawal.deserialize()?,
            })
        }
    }

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
        let _: Vec<ParsedCoin<'_>> = GetCoinsResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| ParsedCoin::try_from(p).unwrap())
            .collect();
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
        let _: Vec<ParsedBalance<'_>> = GetBalancesResponse(json.as_bytes().into())
            .deserialize_partial()
            .unwrap()
            .into_iter()
            .map(|p| ParsedBalance::try_from(p).unwrap())
            .collect();
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
        let _: HashMap<AccountName<'_>, Vec<ParsedBalance<'_>>> =
            GetAllBalancesResponse(json.as_bytes().into())
                .deserialize_partial()
                .unwrap()
                .into_iter()
                .map(|(name, balances)| {
                    (
                        name,
                        balances
                            .into_iter()
                            .map(|b| b.try_into().unwrap())
                            .collect(),
                    )
                })
                .collect();
    }
}
