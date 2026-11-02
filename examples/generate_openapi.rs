//! Generate OpenAPI spec to file
//! Run: cargo run --example generate_openapi

use otakudesu_api::handlers::ApiDoc;
use std::fs;

fn main() {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).expect("serialize openapi");
    fs::write("openapi.json", json).expect("write openapi.json");
    println!("OpenAPI spec written to openapi.json");
}