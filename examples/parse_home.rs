//! Parse home page HTML from file
//! Run: cargo run --example parse_home tests/fixtures/home.html

use otakudesu_api::scrapers::parse_home;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_home <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");
    let result = parse_home(&html);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}