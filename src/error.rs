use core::fmt;
use std::error::Error as CoreError;

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

// A nasty but beautiful implementation that uses memory leaks!!!
// impl CoreError for RurlError {
//     fn source(&self) -> Option<&(dyn CoreError + 'static)> {}
// }
//
// #[derive(Debug)]
// pub enum Error {
//     Generic(&'static dyn CoreError),
// }
// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             // Self::Generic(v) => write!(f, "Generic error::{}", v),
//         }
//     }
// }
//
// impl<T> From<T> for Error
// where
//     T: std::error::Error,
//     T: 'static,
// {
//     fn from(value: T) -> Self {
//         Self::Generic(Box::leak(Box::new(value)))
//     }
// }
