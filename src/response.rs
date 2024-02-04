// use crate::prelude::*;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Response {
    status: u16,
    headers: Option<HashMap<String, String>>,
    body: Option<Value>,
}

impl Response {
    pub fn new(status: u16, headers: Option<HashMap<String, String>>, body: Option<Value>) -> Self {
        Response {
            status,
            headers,
            body,
        }
    }
}

impl Response {
    pub fn build_headers(hdrs: &HeaderMap) -> Option<HashMap<String, String>> {
        let mut headers: HashMap<String, String> = HashMap::new();

        if hdrs.is_empty() {
            return None;
        }

        for (k, v) in hdrs.iter() {
            let k = k.to_string();
            let v = v.to_str();
            // Reqwest can fail to convert a value to a String but not a key
            if let Ok(val) = v {
                headers.insert(k, val.to_string());
            } else {
                // TODO: Output error to stderr
                let e = String::from("Failed to read this fields contents");
                headers.insert(k, e);
            }
        }
        Some(headers)
    }
}
