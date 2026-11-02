//! Parse batch HTML from file
//! Run: cargo run --example parse_batch tests/fixtures/batch-xxx.html

use otakudesu_api::scrapers::parse_batch;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_batch <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");
    let result = parse_batch(&html);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}