use rust_decimal::{serde::arbitrary_precision, Decimal};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    error::Error as StdError,
    fmt,
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    ops::{Add, AddAssign, Mul},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::error::BoxError;

pub type Price = PositiveDecimal;
pub type Size = PositiveDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Exchange<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BaseCurrency<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QuoteCurrency<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Underlying<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountName<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CancelAckMsg<'a>(pub &'a str);

/// Trade or order direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Side {
    pub fn as_param(&self) -> &str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// The time window to consider for some request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowLength {
    FifteenSeconds,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    FourHours,
    Days(WindowLengthDays),
}

impl WindowLength {
    pub fn to_secs(&self) -> u64 {
        use WindowLength::*;

        match self {
            FifteenSeconds => 15,
            OneMinute => 60,
            FiveMinutes => 300,
            FifteenMinutes => 900,
            OneHour => 3600,
            FourHours => 14400,
            Days(multiple) => 86400 * (multiple.0 as u64),
        }
    }
}

/// A multiple of the max window length of one day. Must be less than
/// or equal to 30.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowLengthDays(u8);

impl WindowLengthDays {
    pub fn new(days: NonZeroU8) -> Option<Self> {
        if days.get() > 30 {
            None
        } else {
            Some(Self(days.get()))
        }
    }
}

impl Default for WindowLengthDays {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnixTimestamp(u128);

impl UnixTimestamp {
    pub fn new(ts: u128) -> Self {
        Self(ts)
    }

    pub fn now() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time cannot go backwards")
                .as_millis(),
        )
    }

    pub fn get(&self) -> u128 {
        self.0
    }
}

impl From<u128> for UnixTimestamp {
    fn from(ts: u128) -> Self {
        Self::new(ts)
    }
}

impl TryFrom<f64> for UnixTimestamp {
    type Error = InvalidUnixTimestamp;

    fn try_from(ts: f64) -> Result<Self, Self::Error> {
        if ts.is_sign_negative() {
            Err(ts.into())
        } else {
            Ok((ts as u128).into())
        }
    }
}

impl TryFrom<OffsetDateTime> for UnixTimestamp {
    type Error = InvalidUnixTimestamp;

    fn try_from(dt: OffsetDateTime) -> Result<Self, Self::Error> {
        let ts = dt.unix_timestamp_nanos();

        u128::try_from(ts / 1_000_000)
            .map(Into::into)
            .map_err(|_| ts.into())
    }
}

impl From<UnixTimestamp> for u128 {
    fn from(ts: UnixTimestamp) -> Self {
        ts.0
    }
}

#[derive(Debug, Clone)]
pub enum InvalidUnixTimestamp {
    Int128(i128),
    Float64(f64),
}

impl fmt::Display for InvalidUnixTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int128(ts) => write!(f, "invalid UNIX timestamp {}", ts),
            Self::Float64(ts) => write!(f, "invalid UNIX timestamp {}", ts),
        }
    }
}

impl StdError for InvalidUnixTimestamp {}

impl From<i128> for InvalidUnixTimestamp {
    fn from(ts: i128) -> Self {
        Self::Int128(ts)
    }
}

impl From<f64> for InvalidUnixTimestamp {
    fn from(ts: f64) -> Self {
        Self::Float64(ts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DateTimeStr<'a>(&'a str);

impl<'a> TryFrom<DateTimeStr<'a>> for OffsetDateTime {
    type Error = DateTimeParseError;

    fn try_from(s: DateTimeStr<'a>) -> Result<Self, Self::Error> {
        OffsetDateTime::parse(&s.0, &Rfc3339).map_err(|e| DateTimeParseError::new(s, e))
    }
}

#[derive(Debug)]
pub struct DateTimeParseError {
    dt: String,
    error: BoxError,
}

impl DateTimeParseError {
    fn new(dt: DateTimeStr, error: impl Into<BoxError>) -> Self {
        Self {
            dt: dt.0.into(),
            error: error.into(),
        }
    }
}

impl fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse {} as a datetime", self.dt)
    }
}

impl StdError for DateTimeParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.error.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct PositiveDecimal(Decimal);

impl PositiveDecimal {
    pub fn new(d: Decimal) -> Result<Self, DecimalError> {
        if d.is_sign_positive() {
            Ok(Self(d))
        } else {
            Err(DecimalError::Positive(d))
        }
    }

    pub fn get(&self) -> Decimal {
        self.0
    }
}

impl FromStr for PositiveDecimal {
    type Err = DecimalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = Decimal::from_str(s).map_err(|e| DecimalParseError::Positive {
            s: s.into(),
            error: e.into(),
        })?;

        Self::new(d).map_err(|e| DecimalParseError::Positive {
            s: s.into(),
            error: e.into(),
        })
    }
}

impl<'de> Deserialize<'de> for PositiveDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = arbitrary_precision::deserialize(deserializer)?;

        PositiveDecimal::new(d).map_err(de::Error::custom)
    }
}

impl PartialEq<Decimal> for PositiveDecimal {
    fn eq(&self, other: &Decimal) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<Decimal> for PositiveDecimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Add for PositiveDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for PositiveDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Mul for PositiveDecimal {
    type Output = PositiveDecimal;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl TryFrom<Decimal> for PositiveDecimal {
    type Error = DecimalError;

    fn try_from(d: Decimal) -> Result<Self, Self::Error> {
        Self::new(d)
    }
}

impl From<NonZeroU8> for PositiveDecimal {
    fn from(val: NonZeroU8) -> Self {
        Self(val.get().into())
    }
}

impl From<NonZeroU16> for PositiveDecimal {
    fn from(val: NonZeroU16) -> Self {
        Self(val.get().into())
    }
}

impl From<NonZeroU32> for PositiveDecimal {
    fn from(val: NonZeroU32) -> Self {
        Self(val.get().into())
    }
}

impl From<NonZeroU64> for PositiveDecimal {
    fn from(val: NonZeroU64) -> Self {
        Self(val.get().into())
    }
}

impl From<PositiveDecimal> for Decimal {
    fn from(d: PositiveDecimal) -> Self {
        d.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct NonNegativeDecimal(Decimal);

impl NonNegativeDecimal {
    pub fn new(d: Decimal) -> Result<Self, DecimalError> {
        if d.is_sign_negative() {
            Err(DecimalError::NonNegative(d))
        } else {
            Ok(Self(d))
        }
    }

    pub fn get(&self) -> Decimal {
        self.0
    }
}

impl FromStr for NonNegativeDecimal {
    type Err = DecimalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = Decimal::from_str(s).map_err(|e| DecimalParseError::NonNegative {
            s: s.into(),
            error: e.into(),
        })?;

        Self::new(d).map_err(|e| DecimalParseError::NonNegative {
            s: s.into(),
            error: e.into(),
        })
    }
}

impl<'de> Deserialize<'de> for NonNegativeDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = arbitrary_precision::deserialize(deserializer)?;

        NonNegativeDecimal::new(d).map_err(de::Error::custom)
    }
}

impl PartialEq<Decimal> for NonNegativeDecimal {
    fn eq(&self, other: &Decimal) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<Decimal> for NonNegativeDecimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Add for NonNegativeDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for NonNegativeDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Mul for NonNegativeDecimal {
    type Output = NonNegativeDecimal;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl TryFrom<Decimal> for NonNegativeDecimal {
    type Error = DecimalError;

    fn try_from(d: Decimal) -> Result<Self, Self::Error> {
        Self::new(d)
    }
}

impl From<u8> for NonNegativeDecimal {
    fn from(val: u8) -> Self {
        Self(val.into())
    }
}

impl From<u16> for NonNegativeDecimal {
    fn from(val: u16) -> Self {
        Self(val.into())
    }
}

impl From<u32> for NonNegativeDecimal {
    fn from(val: u32) -> Self {
        Self(val.into())
    }
}

impl From<u64> for NonNegativeDecimal {
    fn from(val: u64) -> Self {
        Self(val.into())
    }
}

impl From<NonNegativeDecimal> for Decimal {
    fn from(d: NonNegativeDecimal) -> Self {
        d.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DecimalError {
    Positive(Decimal),
    NonNegative(Decimal),
}

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive(d) => write!(f, "expected a positive number, got {}", d),
            Self::NonNegative(d) => write!(f, "expected a non-negative number, got {}", d),
        }
    }
}

impl StdError for DecimalError {}

#[derive(Debug)]
pub enum DecimalParseError {
    Positive { s: String, error: BoxError },
    NonNegative { s: String, error: BoxError },
}

impl fmt::Display for DecimalParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive { s, .. } => write!(f, "failed to parse {} as a positive decimal", s),
            Self::NonNegative { s, .. } => {
                write!(f, "failed to parse {} as a non-negative decimal", s)
            }
        }
    }
}

impl StdError for DecimalParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Positive { error, .. } => Some(error.as_ref()),
            Self::NonNegative { error, .. } => Some(error.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn datetime_str_parses() {
        let s = DateTimeStr("2019-03-05T09:56:55.728933+00:00");

        assert_eq!(
            datetime!(2019-03-05 09:56:55.728933 +00:00),
            OffsetDateTime::try_from(s).unwrap()
        )
    }
}
