use crate::{prelude::*, response};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn write_file(path: &str, res: reqwest::blocking::Response) -> Result<()> {
    // Responses are stored in an output directory
    let request_path = Path::new(path).to_path_buf();
    let response_path = build_file_path(request_path)?;

    // Extract our response and convert to our internal struct for serializing to JSON
    let status: u16 = res.status().as_u16();
    let headers = response::Response::build_headers(res.headers());
    let contents: Value = res.json()?; // Convert response body to serde_json::Value for ease
    let resp = response::Response::new(status, headers, Some(contents)); // Pass to Response struct

    let json_contents = serde_json::to_string_pretty(&resp)?; // Format into JSON string
    let file_write_result = fs::write(response_path, json_contents); // Write string to JSON file
    Ok(file_write_result?)
}

// Responses are redirected to a JSON file saved in subdirectoy '/output'
// Construct the dir and return the Response file path to the dir
fn build_file_path(path: PathBuf) -> Result<PathBuf> {
    let parent = path.parent().expect("Failed to find parent directory!");
    let out_dir = parent.join("output");
    println!("{:?}, {:?}", parent, out_dir);
    std::fs::create_dir_all(out_dir)?;
    let filename = path.file_stem().expect("Could not retrieve file name!");
    let ext = path.extension().expect("Couldn't retreive extension!");

    let response_path_str = format!(
        "{}/output/{}_response.{}",
        parent.display(),
        filename.to_str().unwrap(),
        ext.to_str().unwrap()
    );

    let response_path = Path::new(&response_path_str).to_owned();
    println!("{:?}", response_path); // DEBUG
    Ok(response_path)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::build_file_path;
    #[test]
    fn path_name() {
        let resp_path = build_file_path(Path::new("tests/test2.json").to_path_buf()).unwrap();

        // Redundant but useful check
        assert!(Path::new("tests/output").try_exists().unwrap());
        assert_eq!(resp_path, Path::new("tests/output/test2_response.json"));
    }
}
