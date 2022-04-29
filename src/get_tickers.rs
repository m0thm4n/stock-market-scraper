use std::slice::from_raw_parts_mut;
use reqwest;
use tokio;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::prelude::*;

// https://stockanalysis.com/stocks/
#[tokio::main]
pub async fn get_ticker_names() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://stockanalysis.com/stocks/")
        .await?
        .text()
        .await?;

    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("a").unwrap();

    for element in fragment.select(&selector) {
        if element.value().attr("href").contains(&mut "/stocks/") {
            let mut file = File::create("tickers.txt")?;
            file.write_all(element.);
        }
    }

    Ok(())
}
