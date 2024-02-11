use proton::ProtonClient;

#[tokio::test]
async fn test_default() {
    let client = ProtonClient::default();

    let result = client.url().await;
    assert_eq!(result, "http://localhost:8123");
}
