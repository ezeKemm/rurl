use std::{fs, path::Path};

use crate::prelude::*;
use crate::requests::Request;

// TODO: Improve debugging and error handling
pub fn read_file(path: &str) -> Result<Request> {
    let file_path = Path::new(path);
    let data = fs::read_to_string(file_path)?;
    let json: Request = serde_json::from_str(&data)?;
    Ok(json)
}

#[cfg(test)]
mod tests {

    use super::*;
    type Result<E> = core::result::Result<(), E>;

    #[test]
    // Ensure successful de-serialization of a JSON file into internal Request struct
    fn de_json() -> Result<String> {
        let req = read_file("tests/test1.json").unwrap();
        let temp = Request::new("get", "http://httpbin.org/get", None, None);
        assert_eq!(temp, req);
        Ok(())
    }
}
