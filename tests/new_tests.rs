use proton_client::prelude::ProtonClient;

#[tokio::test]
async fn test_new() {
    let url = "https://example.com";
    let client = ProtonClient::new(url);

    let result = client.url().await;
    assert_eq!(result, url);
}
