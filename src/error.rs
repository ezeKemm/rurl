use core::fmt;
use std::error::Error as CoreError;

// Custom error type
#[derive(Debug)]
pub struct RurlError(String);

impl CoreError for RurlError {}

impl RurlError {
    pub(crate) fn new(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for RurlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rurl raised an error: {}", self.0)
    }
}

// Pass errors raised by program into our custom error stored as a string
impl From<std::io::Error> for RurlError {
    fn from(value: std::io::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<serde_json::Error> for RurlError {
    fn from(value: serde_json::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<reqwest::Error> for RurlError {
    fn from(value: reqwest::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<reqwest::header::ToStrError> for RurlError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        Self(value.to_string())
    }
}
