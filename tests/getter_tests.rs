use proton::ProtonClient;

#[tokio::test]
async fn test_client() {
    let url = "https://api.proton.com";
    let client = ProtonClient::new(url);

    let _ = client.client().await;
}

#[tokio::test]
async fn test_url() {

    let url = "https://api.proton.com";
    let client = ProtonClient::new(url);

    let result = client.url().await;
    assert_eq!(result, "https://api.proton.com");
}
