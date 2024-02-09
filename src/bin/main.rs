use clickhouse::{error::Result, Client, Row};
use serde::{Deserialize, Serialize};


#[derive(Debug, Row, Serialize, Deserialize)]
struct MyRow<'a> {
    no: u32,
    name: &'a str,
}

#[derive(Debug, Row, Serialize, Deserialize)]
struct MyRowOwned {
    no: u32,
    name: String,
}

const FN_NAME: &str = "[main]: ";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}start", FN_NAME);

    println!("{}Build client", FN_NAME);
    let client = Client::default().with_url("http://localhost:8123");

    println!("{}Create table", FN_NAME);
    ddl(&client)
        .await
        .expect("[main]: Failed to create table");

    println!("{}Insert data", FN_NAME);
    insert(&client)
        .await
        .expect("[main]: Failed to insert data");

    println!("{}Count inserted data", FN_NAME);
    let count = select_count(&client)
        .await
        .expect("[main/count]: Failed to count inserted data");

    println!("{}count data: {}", FN_NAME, count);

    println!("{}stop", FN_NAME);
    Ok(())
}


async fn ddl(client: &Client) -> Result<()> {
    client
        .query("DROP TABLE IF EXISTS some")
        .execute()
        .await
        .expect("[main/ddl]: Failed to drop table");

    client
        .query(
            "
            CREATE TABLE some(no UInt32, name LowCardinality(String))
            ENGINE = MergeTree
            ORDER BY no
        ",
        )
        .execute()
        .await
}

async fn insert(client: &Client) -> Result<()> {
    let mut insert = client
        .insert("some")
        .expect("[main/insert]: Failed to build inserter for table some");

    for i in 0..1000 {
        insert
            .write(&MyRow { no: i, name: "foo" })
            .await
            .expect("[main/insert]: Failed to insert row into table some");
    }

    insert.end().await
}

async fn select_count(client: &Client) -> Result<u64> {
    let count = client
        .query("SELECT count() FROM some")
        .fetch_one::<u64>()
        .await
        .expect("[main/select_count]: Failed to fetch count()");

    Ok(count)
}