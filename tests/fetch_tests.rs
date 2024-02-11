use clickhouse_derive::Row;
use proton::ProtonClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MyRow {
    name: String,
}

#[tokio::test]
async fn test_fetch() {
    let client = ProtonClient::new("http://localhost:8123");

    let query = "SELECT ?fields from table(some) WHERE no BETWEEN 500 AND 504";

    let result = client.fetch::<MyRow>(query).await;

    assert!(result.is_ok());

    let mut cursor = result.unwrap();

    assert!(cursor.next().await.is_err());
}

#[tokio::test]
async fn test_fetch_all() {
    let client = ProtonClient::new("http://localhost:8123");

    let query = "SELECT ?fields from table(some) WHERE no BETWEEN 500 AND 504";

    let result = client.fetch_all::<MyRow>(query).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_fetch_one() {
    let client = ProtonClient::new("http://localhost:8123");
    let query = "select count() from table(table_name)";
    let result = client.fetch_one::<u64>(query).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_fetch_optional() {
    let client = ProtonClient::new("http://localhost:8123");
    let query = "select count() from table(table_name)";
    let result = client.fetch_optional::<u64>(query).await;

    assert!(result.is_err());
}
