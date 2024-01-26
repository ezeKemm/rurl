use crate::prelude::*;
use reqwest::blocking::Response;
use std::{fs, path::Path};

pub fn write_file(path: &str, res: Response) -> Result<()> {
    println!("yay");
    // Will redirect the data to a new response file in the same folder as the request
    let path: &str = &(path[..path.len() - 5].to_owned() + "_response.json");
    let file_path = Path::new(path);
    // let contents = res.bytes()?;
    let s = res.status().to_owned();
    // let u = res.url();
    let h = res.headers().to_owned();
    let a = res.remote_addr().unwrap().to_owned().to_string();
    let v = res.version();
    // let e = res.extensions();
    let contents = res.text()?;
    let result = fs::write(file_path, contents);
    Ok(result?)
}
