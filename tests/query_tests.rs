use proton::ProtonClient;

#[tokio::test]
async fn test_execute_query() {
    let client = ProtonClient::new("http://localhost:8123");
    let query = "select count() from table(table_name)";

    let result = client.execute_query(query).await;
    assert!(result.is_err());
}
