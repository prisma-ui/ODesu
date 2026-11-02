//! Parse episode HTML from file
//! Run: cargo run --example parse_episode tests/fixtures/episode-xxx.html

use otakudesu_api::scrapers::parse_episode;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_episode <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");
    let result = parse_episode(&html);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}