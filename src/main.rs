use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://searx.garudalinux.org/search?q=solar%20panels&categories=general&format=json&lang=en&page=1")
        .await?
        .json::<RequestData>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestData {
    query: String,
    number_of_results: f32,
    results: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    title: String,
    engine: String,
    content: String,
    pretty_url: String,
}
