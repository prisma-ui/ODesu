//! Parse jadwal rilis HTML from file
//! Run: cargo run --example parse_jadwal_rilis tests/fixtures/jadwal-rilis.html

use otakudesu_api::scrapers::parse_jadwal_rilis;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_jadwal_rilis <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");
    let result = parse_jadwal_rilis(&html);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}