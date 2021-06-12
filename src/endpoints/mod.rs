pub mod account;
pub mod fills;
pub mod funding_payments;
pub mod futures;
pub mod indices;
pub mod markets;
pub mod orders;
pub mod spot_margin;
pub mod statistics;
pub mod subaccounts;
pub mod wallet;

use serde::{de, Deserialize, Deserializer, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    error::Error as StdError,
    fmt,
};

use crate::{
    data::UnixTimestamp,
    error::{Error, ErrorKind},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Success<T> {
    result: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Failure<'a> {
    #[serde(borrow)]
    error: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FtxResponse<'a, T> {
    Success(Success<T>),
    Failure(Failure<'a>),
}

impl<'a, T> FtxResponse<'a, T> {
    pub(crate) fn try_into(self) -> Result<T, Error> {
        match self {
            FtxResponse::Success(success) => Ok(success.result),
            FtxResponse::Failure(failure) => {
                Err(Error::new(ErrorKind::Rejected).with_source(failure.error))
            }
        }
    }
}

impl<'a, T> From<Success<T>> for FtxResponse<'a, T> {
    fn from(val: Success<T>) -> Self {
        Self::Success(val)
    }
}

impl<'a, T> From<Failure<'a>> for FtxResponse<'a, T> {
    fn from(e: Failure<'a>) -> Self {
        Self::Failure(e)
    }
}

impl<'de, T> TryFrom<&'de [u8]> for FtxResponse<'de, T>
where
    T: Deserialize<'de>,
{
    type Error = Error;

    fn try_from(v: &'de [u8]) -> Result<Self, Error> {
        match serde_json::from_slice::<Success<T>>(v) {
            Ok(success) => Ok(success.into()),
            Err(success_parse_err) => match serde_json::from_slice::<Failure>(v) {
                Ok(failure) => Ok(failure.into()),
                Err(failure_parse_err) => Err(Error::new(ErrorKind::DeserializationFailed)
                    .with_source(FtxResponseDeserializationError {
                        success_parse_err,
                        failure_parse_err,
                    })),
            },
        }
    }
}

#[derive(Debug)]
struct FtxResponseDeserializationError {
    success_parse_err: serde_json::Error,
    failure_parse_err: serde_json::Error,
}

impl fmt::Display for FtxResponseDeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to deserialize either a success response ({}) or a failure response ({})",
            self.success_parse_err, self.failure_parse_err
        )
    }
}

impl StdError for FtxResponseDeserializationError {}

fn float_ts_to_unix_ts<'de, D>(deserializer: D) -> Result<UnixTimestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: f64 = Deserialize::deserialize(deserializer)?;
    ts.try_into().map_err(de::Error::custom)
}

mod macros {
    macro_rules! response {
        ($res:ty, $data:ty) => {
            impl From<Bytes> for $res {
                fn from(b: Bytes) -> Self {
                    Self(b)
                }
            }

            impl AsRef<Bytes> for $res {
                fn as_ref(&self) -> &Bytes {
                    &self.0
                }
            }

            impl crate::private::Sealed for $res {}

            impl<'de> crate::Response<'de> for $res {
                type Data = $data;
            }
        };
    }

    pub(super) use response;
}

#[cfg(test)]
mod tests {
    #[test]
    fn url_encoding_works() {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        params.push(("bread", "baguette".into()));

        assert_eq!(
            &serde_urlencoded::to_string(params).unwrap(),
            "bread=baguette"
        );
    }
}
