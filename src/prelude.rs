use crate::error;
use core::result;
pub type Result<T> = result::Result<T, error::RurlError>;
