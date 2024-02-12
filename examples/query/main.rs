use clickhouse::Row;
use proton_client::prelude::Result;
use proton_client::ProtonClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MyRow<'a> {
    no: u32,
    name: &'a str,
}

impl<'a> MyRow<'a> {
    pub fn new(no: u32, name: &'a str) -> Self {
        Self { no, name }
    }
}

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MyRowOwned {
    no: u32,
    name: String,
}

impl MyRowOwned {
    pub fn new(no: u32, name: String) -> Self {
        Self { no, name }
    }
    pub fn no(&self) -> u32 {
        self.no
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

const FN_NAME: &str = "[main]:";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}Build client", FN_NAME);
    let client = ProtonClient::new("http://localhost:8123");

    println!("{}Fetch data", FN_NAME);
    fetch(&client)
        .await
        .expect("[main/fetch]: Failed to fetch data");

    println!("{}Fetch all data", FN_NAME);
    fetch_all(&client)
        .await
        .expect("[main/fetch_all]: Failed to fetch data");

    println!("{}Stop", FN_NAME);
    Ok(())
}

pub async fn fetch(client: &ProtonClient) -> clickhouse::error::Result<()> {
    let mut cursor = client
        .fetch::<MyRow<'_>>("SELECT ?fields from test_stream WHERE no BETWEEN 500 AND 504")
        .await
        .expect("[main/fetch]: Failed to fetch data");

    while let Some(row) = cursor.next().await? {
        println!("{row:?}");
    }

    Ok(())
}

pub async fn fetch_all(client: &ProtonClient) -> clickhouse::error::Result<()> {
    let vec = client
        .fetch_all::<MyRowOwned>("SELECT ?fields from test_stream WHERE no BETWEEN 500 AND 504")
        .await
        .expect("[main/fetch_all]: Failed to fetch all");

    println!("{vec:?}");

    Ok(())
}
