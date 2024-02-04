mod error;
mod prelude;
mod reader;
mod requests;
mod response;
mod sender;
mod writer;

use crate::prelude::*;
use crate::{reader::read_file, sender::send_request, writer::write_file};
use clap::Parser;

#[derive(Parser)]
struct Args {
    request_file_path: String,
    // colorizer: bool,
    // formatter: bool,
}

fn main() {
    let args = Args::parse();
    let e = exec(args);
    if let Err(err) = e {
        println!("{}", err);
    }
}

fn handle_error() {}

fn exec(args: Args) -> Result<()> {
    let request = read_file(&args.request_file_path)?;
    let response = send_request(request)?;
    println!("{:#?}", response);
    let written = write_file(&args.request_file_path, response)?;
    Ok(())
}
