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

use serde::{Deserialize, Deserializer, Serialize};
use std::convert::TryFrom;

use crate::{
    error::{Error, ErrorKind},
    Json,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FtxResponse<'a, T> {
    #[serde(borrow, deserialize_with = "deserialize_some")]
    result: Option<Json<'a, T>>,
    error: Option<&'a str>,
}

impl<'a, T> FtxResponse<'a, T>
where
    T: Deserialize<'a>,
{
    pub(crate) fn deserialize(self) -> Result<T, Error> {
        if let Some(res) = self.result {
            res.deserialize()
                .map_err(|e| Error::new(ErrorKind::DeserializationFailed).with_source(e))
        } else if let Some(err) = self.error {
            Err(Error::new(ErrorKind::RejectedByExchange).with_source(err))
        } else {
            Err(Error::new(ErrorKind::RejectedByExchange))
        }
    }
}

impl<'a, T> TryFrom<&'a [u8]> for FtxResponse<'a, T>
where
    T: Deserialize<'a>,
{
    type Error = Error;

    fn try_from(v: &'a [u8]) -> Result<Self, Error> {
        serde_json::from_slice(v)
            .map_err(|e| Error::new(ErrorKind::DeserializationFailed).with_source(e))
    }
}

fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

mod macros {
    macro_rules! response {
        ($res:ty, $data:ty, $partial_data:ty) => {
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

            impl crate::Response for $res {
                type Data<'a> = $data;

                type PartialData<'a> = $partial_data;
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
