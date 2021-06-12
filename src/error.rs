use hmac::digest::InvalidLength;
use reqwest::StatusCode;
use std::{error::Error as StdError, fmt};

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

pub struct Error(Box<Inner>);

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Error(Box::new(Inner { kind, source: None }))
    }

    pub(crate) fn with_source(mut self, source: impl Into<BoxError>) -> Self {
        self.0.source = Some(source.into());
        self
    }

    pub(crate) fn from_status_code(code: Option<StatusCode>) -> Self {
        if code.map_or(false, |c| c == 429) {
            Error::new(ErrorKind::RateLimitExceeded)
        } else {
            Error::new(ErrorKind::RequestExecutionFailed(code))
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.0.kind)
            .field("source", &self.0.source)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.kind, f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source.as_ref().map(|e| &**e as _)
    }
}

impl From<InvalidLength> for Error {
    fn from(_: InvalidLength) -> Self {
        Error::new(ErrorKind::InvalidKeyLength)
    }
}

struct Inner {
    kind: ErrorKind,
    source: Option<BoxError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidKeyLength,
    InvalidHeaderValue,
    InvalidUrl,
    InvalidPayload,
    RequestBuildFailed,
    RequestExecutionFailed(Option<StatusCode>),
    RateLimitExceeded,
    DeserializationFailed,
    Rejected,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match self {
            InvalidKeyLength => f.write_str("invalid private key length"),
            InvalidHeaderValue => f.write_str("invalid header value"),
            InvalidUrl => f.write_str("invalid URL"),
            InvalidPayload => f.write_str("failed to serialize request payload"),
            RequestBuildFailed => f.write_str("failed to build request"),
            RequestExecutionFailed(ref code) => match code {
                Some(code) => write!(f, "request failed with status code {}", code),
                None => f.write_str("request failed"),
            },
            RateLimitExceeded => f.write_str("rate limits exceeded"),
            DeserializationFailed => f.write_str("failed to deserialize response"),
            Rejected => f.write_str("request rejected by the exchange"),
        }
    }
}
