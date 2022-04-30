use std::collections::HashSet;
use reqwest;
use tokio;
use std::fs::File;
use std::io::prelude::*;
use scraper::{Html, Selector};
use tokio_postgres::Error;
use tokio_postgres::tls::{NoTls};

struct Stock {
    id: i32,
    symbol: String
}

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
                urls.insert(url.replace("/equities/", "").to_string());
            }

        }

        for url in urls{
            add_symbol_to_db(url).await;
            // println!("Added url {} to db", url)
        }

    }

    Ok(())
}

async fn add_symbol_to_db(symbol: String) -> Result<(), Error> {
// Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=Babycakes15! dbname=stocks port=5432 sslmode=disable", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.query("CREATE TABLE stock_symbol (
        id SERIAL PRIMARY KEY,
        symbol VARCHAR NOT NULL
    )", &[]).await;

    println!("Created DB Table");

    let stock = Stock {
        id: 0,
        symbol,
    };

    client.query(
        "INSERT INTO stock_symbol (symbol) VALUES ($1)",
        &[&stock.symbol],
    ).await;

    Ok(())
}