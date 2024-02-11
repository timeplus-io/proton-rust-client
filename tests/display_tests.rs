use proton::ProtonClient;

#[test]
fn test_display() {
    let client = ProtonClient::new("https://api.proton.com");

    let expected = "https://api.proton.com";

    assert_eq!(format!("{}", client), expected);
}
