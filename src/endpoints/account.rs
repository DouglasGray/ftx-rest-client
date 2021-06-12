use bytes::Bytes;
use core::fmt;
use reqwest::Method;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{de, Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    error::Error as StdError,
    num::NonZeroU32,
};

use crate::{
    data::{NonNegativeDecimal, PositiveDecimal, Price, Side, Size},
    private::Sealed,
    QueryParams, Request,
};

use super::macros::response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccountLeverage {
    One,
    Two,
    Three,
    Five,
    Ten,
    Twenty,
}

impl AccountLeverage {
    pub fn as_non_zero_u32(&self) -> NonZeroU32 {
        const ONE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
        const TWO: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(2) };
        const THREE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(3) };
        const FIVE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(5) };
        const TEN: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10) };
        const TWENTY: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(20) };

        match self {
            Self::One => ONE,
            Self::Two => TWO,
            Self::Three => THREE,
            Self::Five => FIVE,
            Self::Ten => TEN,
            Self::Twenty => TWENTY,
        }
    }
}

impl TryFrom<NonZeroU32> for AccountLeverage {
    type Error = InvalidAccountLeverageError;

    fn try_from(value: NonZeroU32) -> Result<Self, Self::Error> {
        Ok(match value.get() {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            5 => Self::Five,
            10 => Self::Ten,
            20 => Self::Twenty,
            _ => return Err(InvalidAccountLeverageError(value)),
        })
    }
}

impl Serialize for AccountLeverage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.as_non_zero_u32().get())
    }
}

impl<'de> Deserialize<'de> for AccountLeverage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f: f64 = Deserialize::deserialize(deserializer)?;

        let u = f
            .to_u32()
            .ok_or_else(|| de::Error::custom(format!("failed to convert float {} to u32", f)))?;

        NonZeroU32::try_from(u)
            .map_err(de::Error::custom)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidAccountLeverageError(NonZeroU32);

impl fmt::Display for InvalidAccountLeverageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid account leverage {}", self.0.get())
    }
}

impl StdError for InvalidAccountLeverageError {}

/// Retrieve account and position information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetAccountInformation;

impl Sealed for GetAccountInformation {}

impl Request<true> for GetAccountInformation {
    const PATH: &'static str = "/account";

    const METHOD: Method = Method::GET;

    type Response = GetAccountInformationResponse;
}

pub struct GetAccountInformationResponse(Bytes);

response!(GetAccountInformationResponse, AccountInformation<'de>);

/// Retrieve current positions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetPositions {
    pub show_avg_price: Option<bool>,
}

impl Sealed for GetPositions {}

impl Request<true> for GetPositions {
    const PATH: &'static str = "/positions";

    const METHOD: Method = Method::GET;

    type Response = GetPositionsResponse;

    fn query_params(&self) -> Option<QueryParams> {
        self.show_avg_price
            .map(|val| vec![("showAvgPrice", val.to_string())])
    }
}

pub struct GetPositionsResponse(Bytes);

response!(GetPositionsResponse, Vec<Position<'de>>);

/// Change an account's leverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub struct ChangeAccountLeverage {
    pub leverage: AccountLeverage,
}

impl Sealed for ChangeAccountLeverage {}

impl Request<true> for ChangeAccountLeverage {
    const PATH: &'static str = "/account/leverage";

    const METHOD: Method = Method::POST;

    type Response = ChangeAccountLeverageResponse;

    fn to_json(&self) -> Option<Result<String, serde_json::Error>> {
        Some(serde_json::to_string(self))
    }
}

pub struct ChangeAccountLeverageResponse(Bytes);

response!(ChangeAccountLeverageResponse, ());

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AccountInformation<'a> {
    pub account_identifier: u64,
    pub account_type: Option<&'a str>,
    pub backstop_provider: bool,
    pub collateral: NonNegativeDecimal,
    pub free_collateral: NonNegativeDecimal,
    pub initial_margin_requirement: NonNegativeDecimal,
    pub maintenance_margin_requirement: NonNegativeDecimal,
    pub leverage: AccountLeverage,
    pub futures_leverage: AccountLeverage,
    pub liquidating: bool,
    pub margin_fraction: Option<NonNegativeDecimal>,
    pub open_margin_fraction: Option<NonNegativeDecimal>,
    pub maker_fee: PositiveDecimal,
    pub taker_fee: PositiveDecimal,
    pub total_account_value: NonNegativeDecimal,
    pub total_position_size: NonNegativeDecimal,
    pub charge_interest_on_negative_usd: bool,
    pub position_limit: Option<PositiveDecimal>,
    pub position_limit_used: Option<NonNegativeDecimal>,
    pub use_ftt_collateral: bool,
    pub username: &'a str,
    pub spot_lending_enabled: bool,
    pub spot_margin_enabled: bool,
    pub spot_margin_withdrawals_enabled: bool,
    #[serde(borrow)]
    pub positions: Vec<Position<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Position<'a> {
    pub cost: Decimal,
    pub entry_price: Option<Price>,
    pub estimated_liquidation_price: Option<Price>,
    #[serde(borrow)]
    pub future: &'a str,
    pub initial_margin_requirement: PositiveDecimal,
    pub maintenance_margin_requirement: PositiveDecimal,
    pub long_order_size: Size,
    pub short_order_size: Size,
    pub net_size: Decimal,
    pub open_size: NonNegativeDecimal,
    pub realized_pnl: Decimal,
    pub side: Side,
    pub size: Size,
    pub unrealized_pnl: Decimal,
    pub collateral_used: NonNegativeDecimal,
    pub recent_average_open_price: Option<Price>,
    pub recent_break_even_price: Option<Price>,
    pub recent_pnl: Option<Decimal>,
    pub cumulative_buy_size: Option<NonNegativeDecimal>,
    pub cumulative_sell_size: Option<NonNegativeDecimal>,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[test]
    fn get_account_information() {
        let json = r#"
{
  "result": {
    "accountIdentifier": 1338857,
    "accountType": null,
    "backstopProvider": false,
    "chargeInterestOnNegativeUsd": false,
    "collateral": 3.859272138279288,
    "freeCollateral": 3.859272138279288,
    "futuresLeverage": 3.0,
    "initialMarginRequirement": 0.33333333,
    "leverage": 3.0,
    "liquidating": false,
    "maintenanceMarginRequirement": 0.03,
    "makerFee": 0.00019,
    "takerFee":0.000665,
    "totalAccountValue": 3568180.98341129,
    "totalPositionSize": 6384939.6992,
    "marginFraction": null,
    "openMarginFraction": null,
    "positionLimit": null,
    "positionLimitUsed": null,
    "useFttCollateral": false,
    "spotLendingEnabled": true,
    "spotMarginEnabled": true,
    "spotMarginWithdrawalsEnabled": true,
    "username": "user@domain.com",
    "positions": [
      {
        "collateralUsed": 0,
        "cost": 0,
        "entryPrice": null,
        "estimatedLiquidationPrice": null,
        "future": "VET-PERP",
        "initialMarginRequirement": 0.33333333,
        "longOrderSize": 0,
        "maintenanceMarginRequirement": 0.03,
        "netSize": 0,
        "openSize": 0,
        "realizedPnl": -5.2667467,
        "shortOrderSize": 0,
        "side": "buy",
        "size": 0,
        "unrealizedPnl": 0
      }
    ]
  }
}
"#;
        GetAccountInformationResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }

    #[test]
    fn get_positions() {
        let json = r#"
{
  "result": [
    {
      "collateralUsed": 0,
      "cost": 0,
      "cumulativeBuySize": null,
      "cumulativeSellSize": null,
      "entryPrice": null,
      "estimatedLiquidationPrice": null,
      "future": "VET-PERP",
      "initialMarginRequirement": 0.33333333,
      "longOrderSize": 0,
      "maintenanceMarginRequirement": 0.03,
      "netSize": 0,
      "openSize": 0,
      "realizedPnl": -5.2667467,
      "recentAverageOpenPrice": null,
      "recentBreakEvenPrice": null,
      "recentPnl": null,
      "shortOrderSize": 0,
      "side": "buy",
      "size": 0,
      "unrealizedPnl": 0
    }
  ]
}
"#;
        GetPositionsResponse(json.as_bytes().into())
            .to_data()
            .unwrap();
    }
}
