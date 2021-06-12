use hmac::{digest::InvalidLength, Hmac, Mac};
use reqwest::{header::HeaderMap, Method};
use sha2::Sha256;
use std::convert::{TryFrom, TryInto};
use urlencoding;

use crate::{
    data::UnixTimestamp,
    error::{Error, ErrorKind},
};

const FTX_KEY_HEADER: &'static str = "FTX-KEY";
const FTX_SIGN_HEADER: &'static str = "FTX-SIGN";
const FTX_TS_HEADER: &'static str = "FTX-TS";
const FTX_SUBACCOUNT_HEADER: &'static str = "FTX-SUBACCOUNT";

#[derive(Clone)]
pub struct PrivateKey(String);

impl PrivateKey {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl<T: Into<String>> From<T> for PrivateKey {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl TryFrom<PrivateKey> for Hmac<Sha256> {
    type Error = InvalidLength;

    fn try_from(key: PrivateKey) -> Result<Self, Self::Error> {
        Hmac::<Sha256>::new_from_slice(key.0.as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey(String);

impl PublicKey {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl<T: Into<String>> From<T> for PublicKey {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Clone)]
pub struct Subaccount(String);

impl Subaccount {
    pub fn new(nickname: impl Into<String>) -> Self {
        Self(nickname.into())
    }
}

impl<T: Into<String>> From<T> for Subaccount {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

#[derive(Clone)]
pub struct Authenticator {
    hmac: Hmac<Sha256>,
    base_headers: HeaderMap,
}

impl Authenticator {
    pub fn new(
        public_key: PublicKey,
        private_key: PrivateKey,
        subaccount: Option<Subaccount>,
    ) -> Result<Self, Error> {
        let hmac = private_key.try_into()?;

        let mut base_headers = HeaderMap::with_capacity(2);

        add_header_value(FTX_KEY_HEADER, &public_key.0, &mut base_headers)?;

        if let Some(s) = subaccount {
            add_header_value(
                FTX_SUBACCOUNT_HEADER,
                &urlencoding::encode(&s.0),
                &mut base_headers,
            )?;
        }

        Ok(Self { hmac, base_headers })
    }

    pub(crate) fn generate_auth_headers(
        &self,
        timestamp: UnixTimestamp,
        method: &Method,
        path: &str,
        payload: Option<&str>,
    ) -> Result<HeaderMap, Error> {
        let signature = sign(self.hmac.clone(), timestamp, method, path, payload)?;

        let mut headers = self.base_headers.clone();

        add_header_value(FTX_SIGN_HEADER, &signature, &mut headers)?;
        add_header_value(FTX_TS_HEADER, &timestamp.get().to_string(), &mut headers)?;

        Ok(headers)
    }
}

fn sign(
    mut hmac: Hmac<Sha256>,
    timestamp: UnixTimestamp,
    method: &Method,
    path: &str,
    payload: Option<&str>,
) -> Result<String, Error> {
    let body = payload.unwrap_or("");

    let to_sign = format!("{}{}/api{}{}", timestamp.get(), method, path, body);

    hmac.update(to_sign.as_bytes());
    Ok(hex::encode(&hmac.finalize().into_bytes()))
}

fn add_header_value(
    header: &'static str,
    val: &str,
    header_map: &mut HeaderMap,
) -> Result<(), Error> {
    let header_val = val.parse().map_err(|_| {
        Error::new(ErrorKind::InvalidHeaderValue)
            .with_source(format!("could not parse {} as header value", header))
    })?;

    header_map.insert(header, header_val);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_for_get_request_should_be_correct() {
        let private_key = PrivateKey::new("YAGN-Np3au9igIMqIAPiJTF1zy9heo55_FNfYEru");
        let path = "/spot_margin/borrow_rates";
        let timestamp = UnixTimestamp::new(1617659558822);

        let signature = sign(
            private_key.try_into().unwrap(),
            timestamp,
            &Method::GET,
            path,
            None,
        )
        .unwrap();

        assert_eq!(
            signature,
            "421c580094ab840e832071c75602f1f7d1504901175589284e6ce81ff163ec0b"
        );
    }

    #[test]
    fn signature_for_post_request_should_be_correct() {
        let private_key = PrivateKey::new("T4lPid48QtjNxjLUFOcUZghD7CUJ7sTVsfuvQZF2");
        let path = "/orders";
        let timestamp = UnixTimestamp::new(1588591856950);

        let request_body = r#"{"market": "BTC-PERP", "side": "buy", "price": 8500, "size": 1, "type": "limit", "reduceOnly": false, "ioc": false, "postOnly": false, "clientId": null}"#;

        let signature = sign(
            private_key.try_into().unwrap(),
            timestamp,
            &Method::POST,
            path,
            Some(&request_body),
        )
        .unwrap();

        assert_eq!(
            signature,
            "c4fbabaf178658a59d7bbf57678d44c369382f3da29138f04cd46d3d582ba4ba"
        );
    }
}
