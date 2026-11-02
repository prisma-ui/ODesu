//! Parse anime grid (ongoing/complete) HTML from file
//! Run: cargo run --example parse_anime_grid tests/fixtures/ongoing.html

use otakudesu_api::scrapers::parse_anime_grid;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_anime_grid <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");
    let result = parse_anime_grid(&html);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}