use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// use crate::prelude::*;

// Built as recommended in Section 2.2.1 [RFC9112] (https://www.rfc-editor.org/rfc/rfc9112.html#section-2.2-1)
// TODO: Integrate extensions into this?
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Request {
    method: String,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<Value>,
}

impl Request {
    // The linter says this code is never used but its in the test case so its wrong
    #[allow(dead_code)]
    pub fn new(
        method: &str,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Self {
        Request {
            method: method.to_string(),
            url: url.to_string(),
            headers,
            body,
        }
    }

    // Values don't need to be wrapped in Result because they're guranteed to exist if Serde does
    // not raise an error
    // Serde however does not validate the validity of the data passed (i.e. a malformed request)
    pub fn url(&self) -> String {
        self.url.to_owned()
    }
    pub fn method(&self) -> String {
        self.method.to_owned()
    }

    // Since reqwest basically ignores an empty field, we will pass this instead of raising an
    // error
    pub fn headers(&self) -> HashMap<String, String> {
        if let Some(headers) = &self.headers {
            headers.clone()
        } else {
            HashMap::new()
        }
    }
    pub fn body(&self) -> Value {
        if let Some(body) = &self.body {
            body.clone()
        } else {
            serde_json::Value::default()
        }
    }
}
