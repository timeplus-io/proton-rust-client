use clickhouse_derive::Row;
use proton_client::ProtonClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MyRow {
    name: String,
}

#[tokio::test]
async fn test_insert() {
    let client = ProtonClient::new("http://localhost:8123");

    let insert = client.insert::<MyRow>("table_name").await;
    assert!(insert.is_ok());
}

#[tokio::test]
async fn test_inserter() {
    let client = ProtonClient::new("http://localhost:8123");

    let insert = client.inserter::<MyRow>("table_name").await;
    assert!(insert.is_ok());
}
