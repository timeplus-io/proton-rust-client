use proton::prelude::ProtonClient;

#[test]
fn test_new() {
    let url = "https://example.com";
    let client = ProtonClient::new(url);
    assert_eq!(client.url(), url);
}
