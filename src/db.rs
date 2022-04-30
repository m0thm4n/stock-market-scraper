use tokio_postgres::Error;
use tokio_postgres::tls::{NoTls};

struct Stock {
    id: i32,
    symbol: String
}

#[tokio::main]
pub async fn add_symbol_to_db(symbol: String) -> Result<(), Error> {
// Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute("CREATE TABLE stock (
        id: SERIAL PRIMARY KEY,
        symbol  VARCHAR NOT NULL
    )", &[]).await?;

    let stock = Stock {
        id: 0,
        symbol,
    };

    client.execute(
        "INSERT INTO stock (symbol) VALUES ({})",
        &[&stock.symbol],
    ).await?;

    Ok(())
}