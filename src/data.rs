use serde::{de, ser, Deserialize, Deserializer, Serialize};
use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt,
    num::NonZeroU8,
    time::{SystemTime, UNIX_EPOCH},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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

/// Future type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutureType {
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "prediction")]
    Prediction,
}

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

    pub fn from_system_time() -> Self {
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

impl From<UnixTimestamp> for u128 {
    fn from(ts: UnixTimestamp) -> Self {
        ts.0
    }
}

impl TryFrom<OffsetDateTime> for UnixTimestamp {
    type Error = InvalidUnixTimestamp;

    fn try_from(dt: OffsetDateTime) -> Result<Self, Self::Error> {
        let ts = dt.unix_timestamp_nanos();

        u128::try_from(ts / 1_000_000)
            .map(Into::into)
            .map_err(|_| InvalidUnixTimestamp::Int128(ts))
    }
}

impl TryFrom<i64> for UnixTimestamp {
    type Error = InvalidUnixTimestamp;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value.is_negative() {
            Err(InvalidUnixTimestamp::Int128(value.into()))
        } else {
            Ok((value as u128).into())
        }
    }
}

impl TryFrom<i128> for UnixTimestamp {
    type Error = InvalidUnixTimestamp;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value.is_negative() {
            Err(InvalidUnixTimestamp::Int128(value))
        } else {
            Ok((value as u128).into())
        }
    }
}

#[derive(Debug, Clone)]
pub enum InvalidUnixTimestamp {
    Int128(i128),
}

impl fmt::Display for InvalidUnixTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int128(ts) => write!(f, "invalid UNIX timestamp {}", ts),
        }
    }
}

impl StdError for InvalidUnixTimestamp {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FtxDateTime(OffsetDateTime);

impl FtxDateTime {
    pub fn get(&self) -> OffsetDateTime {
        self.0
    }
}

impl<'de> Deserialize<'de> for FtxDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        OffsetDateTime::parse(s, &Rfc3339)
            .map_err(de::Error::custom)
            .map(Self)
    }
}

impl Serialize for FtxDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.0.format(&Rfc3339).map_err(ser::Error::custom)?;
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn deserialize_datetime_str() {
        let s = r#"["2019-03-05T09:56:55.728933+00:00"]"#;

        let de: [FtxDateTime; 1] = serde_json::from_str(s).unwrap();

        // Confirm datetime matches
        assert_eq!(datetime!(2019-03-05 09:56:55.728933 +00:00), de[0].0);
    }
}
