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
        let s = format!("StdIoError: {}", value);
        Self(s)
    }
}

impl From<std::num::ParseIntError> for RurlError {
    fn from(value: std::num::ParseIntError) -> Self {
        let s = format!("ParseIntError: {}", value);
        Self(s)
    }
}

impl From<serde_json::Error> for RurlError {
    fn from(value: serde_json::Error) -> Self {
        let s = format!("SerdeError: {}", value);
        Self(s)
    }
}

impl From<reqwest::Error> for RurlError {
    fn from(value: reqwest::Error) -> Self {
        let s = format!("reqwestError: {}", value);
        Self(s)
    }
}

impl From<reqwest::header::ToStrError> for RurlError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        let s = format!("reqwest::header::ToStrError: {}", value);
        Self(s)
    }
}
