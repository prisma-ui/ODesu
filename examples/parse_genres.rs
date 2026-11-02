//! Parse genres HTML from file
//! Run: cargo run --example parse_genres tests/fixtures/genres.html

use otakudesu_api::scrapers::{parse_genre_list, parse_genre_detail};
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parse_genres <html_file>");
    let html = fs::read_to_string(&path).expect("read html file");

    // Try both parsers
    if let Ok(list) = std::panic::catch_unwind(|| parse_genre_list(&html)) {
        println!("=== Genre List ===");
        println!("{}", serde_json::to_string_pretty(&list).unwrap());
    }

    if let Ok(detail) = std::panic::catch_unwind(|| parse_genre_detail(&html)) {
        println!("=== Genre Detail ===");
        println!("{}", serde_json::to_string_pretty(&detail).unwrap());
    }
}