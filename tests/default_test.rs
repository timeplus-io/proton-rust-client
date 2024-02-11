use proton::ProtonClient;

#[test]
fn test_default() {
    let client = ProtonClient::default();

    assert_eq!(client.url(), "http://localhost:8123");

}