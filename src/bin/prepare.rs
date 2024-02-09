use clickhouse::Row;
use serde::{Deserialize, Serialize};
use proton::prelude::{Result, ProtonClient};

const FN_NAME: &str = "[prepare]:";

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("{} Start", FN_NAME);

    println!("{}Build client", FN_NAME);
    let client = ProtonClient::new("http://localhost:8123");

    println!("{} Create stream if not exists", FN_NAME);
    create_stream(&client)
        .await
        .expect("[main]: Failed to create Stream");

    println!("{}Insert data", FN_NAME);
    insert(&client)
        .await
        .expect("[main]: Failed to insert data");

    println!("{}Count inserted data", FN_NAME);
    let count = select_count(&client)
        .await
        .expect("[main/count]: Failed to count inserted data");

    println!("{}Inserted data: {}", FN_NAME, count);

    println!("{} Stop", FN_NAME);
    Ok(())
}

pub async fn create_stream(client: &ProtonClient) -> Result<()> {
    client
        .execute("CREATE STREAM IF NOT EXISTS some(no uint32, name string) ORDER BY no")
        .await
}

pub async fn insert(client: &ProtonClient) -> Result<()> {
    let mut insert = client
        .insert("some")
        .await
        .expect("[main/insert]: Failed to build inserter for table some");

    for i in 0..1000 {
        insert
            .write(&MyRow::new(i, "foo"))
            .await
            .expect("[main/insert]: Failed to insert row into table some");
    }

    insert.end().await.expect("TODO: panic message");

    Ok(())
}

pub async fn select_count(client: &ProtonClient) -> Result<u64> {
    let count = client.clone()
        .fetch_one("select count() from table(some)")
        .await
        .expect("[main/select_count]: Failed to fetch count()");

    Ok(count)
}