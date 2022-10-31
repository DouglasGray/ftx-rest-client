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

use crate::{data::Side, private::Sealed, Json, OptJson, QueryParams, Request};

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

response!(GetAccountInformationResponse, AccountInformation<'a>);

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

response!(GetPositionsResponse, Vec<Position<'a>>);

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AccountInformation<'a> {
    #[serde(borrow)]
    pub account_identifier: Json<'a, u64>,
    pub account_type: Option<&'a str>,
    #[serde(borrow)]
    pub backstop_provider: Json<'a, bool>,
    #[serde(borrow)]
    pub collateral: Json<'a, Decimal>,
    #[serde(borrow)]
    pub free_collateral: Json<'a, Decimal>,
    #[serde(borrow)]
    pub initial_margin_requirement: Json<'a, Decimal>,
    #[serde(borrow)]
    pub maintenance_margin_requirement: Json<'a, Decimal>,
    #[serde(borrow)]
    pub leverage: Json<'a, AccountLeverage>,
    #[serde(borrow)]
    pub futures_leverage: Json<'a, AccountLeverage>,
    #[serde(borrow)]
    pub liquidating: Json<'a, bool>,
    #[serde(borrow)]
    pub margin_fraction: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub open_margin_fraction: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub maker_fee: Json<'a, Decimal>,
    #[serde(borrow)]
    pub taker_fee: Json<'a, Decimal>,
    #[serde(borrow)]
    pub total_account_value: Json<'a, Decimal>,
    #[serde(borrow)]
    pub total_position_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub charge_interest_on_negative_usd: Json<'a, bool>,
    #[serde(borrow)]
    pub position_limit: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub position_limit_used: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub use_ftt_collateral: Json<'a, bool>,
    pub username: &'a str,
    #[serde(borrow)]
    pub spot_lending_enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub spot_margin_enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub spot_margin_withdrawals_enabled: Json<'a, bool>,
    #[serde(borrow)]
    pub positions: Vec<Position<'a>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Position<'a> {
    #[serde(borrow)]
    pub cost: Json<'a, Decimal>,
    #[serde(borrow)]
    pub entry_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub estimated_liquidation_price: OptJson<'a, Decimal>,
    pub future: &'a str,
    #[serde(borrow)]
    pub initial_margin_requirement: Json<'a, Decimal>,
    #[serde(borrow)]
    pub maintenance_margin_requirement: Json<'a, Decimal>,
    #[serde(borrow)]
    pub long_order_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub short_order_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub net_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub open_size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub realized_pnl: Json<'a, Decimal>,
    #[serde(borrow)]
    pub side: Json<'a, Side>,
    #[serde(borrow)]
    pub size: Json<'a, Decimal>,
    #[serde(borrow)]
    pub unrealized_pnl: Json<'a, Decimal>,
    #[serde(borrow)]
    pub collateral_used: Json<'a, Decimal>,
    #[serde(borrow)]
    pub recent_average_open_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub recent_break_even_price: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub recent_pnl: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub cumulative_buy_size: OptJson<'a, Decimal>,
    #[serde(borrow)]
    pub cumulative_sell_size: OptJson<'a, Decimal>,
}

#[cfg(test)]
mod tests {
    use crate::Response;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    struct ParsedAccountInformation<'a> {
        pub account_identifier: u64,
        pub account_type: Option<&'a str>,
        pub backstop_provider: bool,
        pub collateral: Decimal,
        pub free_collateral: Decimal,
        pub initial_margin_requirement: Decimal,
        pub maintenance_margin_requirement: Decimal,
        pub leverage: AccountLeverage,
        pub futures_leverage: AccountLeverage,
        pub liquidating: bool,
        pub margin_fraction: Option<Decimal>,
        pub open_margin_fraction: Option<Decimal>,
        pub maker_fee: Decimal,
        pub taker_fee: Decimal,
        pub total_account_value: Decimal,
        pub total_position_size: Decimal,
        pub charge_interest_on_negative_usd: bool,
        pub position_limit: Option<Decimal>,
        pub position_limit_used: Option<Decimal>,
        pub use_ftt_collateral: bool,
        pub username: &'a str,
        pub spot_lending_enabled: bool,
        pub spot_margin_enabled: bool,
        pub spot_margin_withdrawals_enabled: bool,
        pub positions: Vec<ParsedPosition<'a>>,
    }

    impl<'a> TryFrom<AccountInformation<'a>> for ParsedAccountInformation<'a> {
        type Error = serde_json::Error;

        fn try_from(v: AccountInformation<'a>) -> Result<Self, Self::Error> {
            Ok(ParsedAccountInformation {
                account_identifier: v.account_identifier.deserialize()?,
                account_type: v.account_type,
                backstop_provider: v.backstop_provider.deserialize()?,
                collateral: v.collateral.deserialize()?,
                free_collateral: v.free_collateral.deserialize()?,
                initial_margin_requirement: v.initial_margin_requirement.deserialize()?,
                maintenance_margin_requirement: v.maintenance_margin_requirement.deserialize()?,
                leverage: v.leverage.deserialize()?,
                futures_leverage: v.futures_leverage.deserialize()?,
                liquidating: v.liquidating.deserialize()?,
                margin_fraction: v.margin_fraction.deserialize()?,
                open_margin_fraction: v.open_margin_fraction.deserialize()?,
                maker_fee: v.maker_fee.deserialize()?,
                taker_fee: v.taker_fee.deserialize()?,
                total_account_value: v.total_account_value.deserialize()?,
                total_position_size: v.total_position_size.deserialize()?,
                charge_interest_on_negative_usd: v.charge_interest_on_negative_usd.deserialize()?,
                position_limit: v.position_limit.deserialize()?,
                position_limit_used: v.position_limit_used.deserialize()?,
                use_ftt_collateral: v.use_ftt_collateral.deserialize()?,
                username: v.username,
                spot_lending_enabled: v.spot_lending_enabled.deserialize()?,
                spot_margin_enabled: v.spot_margin_enabled.deserialize()?,
                spot_margin_withdrawals_enabled: v.spot_margin_withdrawals_enabled.deserialize()?,
                positions: v
                    .positions
                    .into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
    struct ParsedPosition<'a> {
        pub cost: Decimal,
        pub entry_price: Option<Decimal>,
        pub estimated_liquidation_price: Option<Decimal>,
        pub future: &'a str,
        pub initial_margin_requirement: Decimal,
        pub maintenance_margin_requirement: Decimal,
        pub long_order_size: Decimal,
        pub short_order_size: Decimal,
        pub net_size: Decimal,
        pub open_size: Decimal,
        pub realized_pnl: Decimal,
        pub side: Side,
        pub size: Decimal,
        pub unrealized_pnl: Decimal,
        pub collateral_used: Decimal,
        pub recent_average_open_price: Option<Decimal>,
        pub recent_break_even_price: Option<Decimal>,
        pub recent_pnl: Option<Decimal>,
        pub cumulative_buy_size: Option<Decimal>,
        pub cumulative_sell_size: Option<Decimal>,
    }

    impl<'a> TryFrom<Position<'a>> for ParsedPosition<'a> {
        type Error = serde_json::Error;

        fn try_from(p: Position<'a>) -> Result<Self, Self::Error> {
            Ok(ParsedPosition {
                cost: p.cost.deserialize()?,
                entry_price: p.entry_price.deserialize()?,
                estimated_liquidation_price: p.estimated_liquidation_price.deserialize()?,
                future: p.future,
                initial_margin_requirement: p.initial_margin_requirement.deserialize()?,
                maintenance_margin_requirement: p.maintenance_margin_requirement.deserialize()?,
                long_order_size: p.long_order_size.deserialize()?,
                short_order_size: p.short_order_size.deserialize()?,
                net_size: p.net_size.deserialize()?,
                open_size: p.open_size.deserialize()?,
                realized_pnl: p.realized_pnl.deserialize()?,
                side: p.side.deserialize()?,
                size: p.size.deserialize()?,
                unrealized_pnl: p.unrealized_pnl.deserialize()?,
                collateral_used: p.collateral_used.deserialize()?,
                recent_average_open_price: p.recent_average_open_price.deserialize()?,
                recent_break_even_price: p.recent_break_even_price.deserialize()?,
                recent_pnl: p.recent_pnl.deserialize()?,
                cumulative_buy_size: p.cumulative_buy_size.deserialize()?,
                cumulative_sell_size: p.cumulative_sell_size.deserialize()?,
            })
        }
    }

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
        let _: ParsedAccountInformation<'_> = GetAccountInformationResponse(json.as_bytes().into())
            .parse()
            .unwrap()
            .try_into()
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

        let _: Vec<ParsedPosition<'_>> = GetPositionsResponse(json.as_bytes().into())
            .parse()
            .unwrap()
            .into_iter()
            .map(|p| ParsedPosition::try_from(p).unwrap())
            .collect();
    }
}
