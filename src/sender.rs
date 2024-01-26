use std::error::Error;
use std::str::FromStr;

use crate::error::RurlError;
use crate::prelude::*;
use crate::requests;

use reqwest::{
    blocking::{self, Response},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
// TODO: Split Request building and request sending

// Generates and sends an HTTP Request using reqwest and our deserialized request from our JSON file
// Returns a Response object from reqwest crate
pub fn send_request(req: requests::Request) -> Result<Response> {
    // No async for convenience, usage is not high enough traffic for concern
    let client = blocking::Client::new();

    // Gather requisite parts for HTTP Request: start line, headers, and body
    // Method object is case-sensitive bc the string is passed as bytes -> uppercase string
    let method = Method::from_bytes(req.method().to_uppercase().as_bytes())
        // To avoid implementing FromResidual for error handling, map to a new instance of RurlError
        .map_err(|e| crate::error::RurlError::new(e.to_string()))?;
    let url = req.url();
    let headers = req.headers();
    let body = req.body();

    // Need to implement HashMap into HeaderMap to neatly add headers to request
    // Construct headers into reqwest HeaderMap
    let hdrmap = headermap_from_hashmap(headers.iter());
    println!("request line: {} / {}", method, url);

    // Build request from API with HTTP Method and target url attached
    let mut request = client.request(method, url);
    request = request.headers(hdrmap); // Attach headers
    request = request.json(&body); // Attach body as json
    println!("request: {:#?}", request);

    // Build our Request struct to send
    // let re = request.build()?;
    // Send request, return Response
    let res = request.send()?;
    // println!("{:#?}", res);
    // match res {
    //     Ok(response) => Ok(response),
    //     Err(e) => {
    //         let i = e.source();
    //         // panic!("{:#?} / {}", e, i.unwrap());
    //         Err(RurlError::new(e.to_string()))
    //     }
    // }
    // let res = client.execute(re)?;
    Ok(res)
}

// TODO: Implement via TryFrom for HeaderName and HeaderValue in http crate
// Implementation of Hashmap into HeaderMap sourced from: https://github.com/seanmonstar/reqwest/issues/555
fn headermap_from_hashmap<'a, I, S>(headers: I) -> HeaderMap
where
    I: Iterator<Item = (S, S)> + 'a,
    S: AsRef<str> + 'a,
{
    headers
        .map(|(name, val)| {
            (
                HeaderName::from_str(name.as_ref()),
                HeaderValue::from_str(val.as_ref()),
            )
        })
        // We ignore the errors here. If you want to get a list of failed conversions, you can use Iterator::partition
        // to help you out here
        // TODO: Implement notices to user about failed conversion
        .filter(|(k, v)| k.is_ok() && v.is_ok())
        .map(|(k, v)| (k.unwrap(), v.unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    // type Result<E> = core::result::Result<(), E>;
    #[test]
    // Ensure send_request successfully returns a Response
    // Return a 200 Status but the request can still possibly be malformed
    fn send_json_get_request() {
        let req = crate::read_file("tests/test1.json");
        let res = send_request(req.unwrap());
        if let Ok(response) = res {
            let status_code = response.status().as_u16();
            assert_eq!(status_code, 200)
        } else {
            panic!("send_json_get_request did not unpack a value")
        }
    }

    #[test]
    // Testing that the helper function correctly builds a HeaderMap
    fn headermap() {
        use reqwest::header;
        use std::collections::HashMap;

        let mut hash_test: HashMap<String, String> = HashMap::new();
        hash_test.insert("content-type".to_string(), "application/json".to_string());
        hash_test.insert(
            "authorization".parse().unwrap(),
            "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string(),
        );
        hash_test.insert("accept".to_string(), "application/json".to_string());

        let mut their_map = HeaderMap::new();
        their_map.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        their_map.insert(
            header::AUTHORIZATION,
            "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
                .parse()
                .unwrap(),
        );
        their_map.insert(header::ACCEPT, "application/json".parse().unwrap());

        let headermap = headermap_from_hashmap(hash_test.iter());
        assert!(headermap.eq(&their_map))
    }

    #[test]
    // Ensure a malformed HTTP request does not panic but does return an Error Code
    fn bad_method() {
        let req = crate::read_file("tests/test3.json");
        let res = send_request(req.unwrap());
        if let Ok(response) = res {
            let status_code = response.status().as_u16();
            assert_ne!(status_code, 200)
        }
    }
    #[test]
    // Returns error if url is malformed
    fn bad_url() {
        let req = crate::reader::read_file("tests/test2.json");
        let res = send_request(req.unwrap());
        assert!(res.is_err())
    }

    #[test]
    #[should_panic]
    // Poor formating of the file results in Serde panicking
    fn malformed_file() {
        let req = crate::read_file("tests/test4.json");
        let _ = send_request(req.unwrap());
    }
}
