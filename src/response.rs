use crate::prelude::*;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Response {
    status: u16,
    headers: Option<HashMap<String, String>>,
    version: String,
    body: String,
}

impl Response {
    pub fn new(
        status: u16,
        headers: Option<HashMap<String, String>>,
        version: String,
        body: String,
    ) -> Self {
        Response {
            status,
            version,
            headers,
            body,
        }
    }
}

fn build_headers(hdrs: Option<HeaderMap>) -> Result<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();

    match hdrs {
        Some(hdrs) => {
            for (k, v) in hdrs.iter() {
                let k = k.to_string();
                let v = v.to_str();
                // Reqwest
                if let Ok(val) = v {
                    headers.insert(k, val.to_string());
                } else {
                    let e = String::from("Failed to read this fields contents");
                    headers.insert(k, e);
                }
            }
        }
        // TODO: Don't think this can ever happen
        None => {}
    }
    Ok(headers)
}
