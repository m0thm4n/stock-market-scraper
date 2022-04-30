use std::collections::HashSet;
use reqwest;
use tokio;
use std::fs::File;
use std::io::prelude::*;
use scraper::{Html, Selector};
use crate::db::add_symbol_to_db;

// https://stockanalysis.com/stocks/
// https://eoddata.com/stocklist/NYSE/
#[tokio::main]
pub async fn get_ticker_names() -> Result<(), Box<dyn std::error::Error>> {
    let mut counter = 1;

    while counter <= 106 {
        let mut url = "https://swingtradebot.com/equities?page=".to_owned() + &counter.to_string();

        println!("{}", &*url);

        let body = reqwest::get(&url)
            .await?
            .text()
            .await?;

        counter += 1;

        let mut urls: HashSet<String> = HashSet::new();

        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"table > tbody > tr > td > a"#).unwrap();

        for title in document.select(&selector) {
            let url = title.value().attr("href").expect("href not found").to_string();
            if url != "/" || url != "." || url != ".." {
                urls.insert(url.replace("/equities/", ""));
                add_symbol_to_db(url.replace("/equities/", ""));
            }

        }

        for url in urls{
            println!("{}", url);
        }

    }

    Ok(())
}
