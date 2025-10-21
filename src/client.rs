use dotenv::dotenv;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::env;
use tokio_postgres::{Client, Error};

pub async fn connect_to_db() -> Result<Client, Error> {
    dotenv().ok(); // load .env file if present

    let url = env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL must be set in .env or environment");

    let connector = TlsConnector::builder().build().unwrap();
    let tls = MakeTlsConnector::new(connector);

    let (client, connection) = tokio_postgres::connect(&url, tls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
