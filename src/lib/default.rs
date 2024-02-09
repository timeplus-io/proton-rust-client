use crate::ProtonClient;

impl Default for ProtonClient {
    fn default() -> Self {
        Self::new("http://localhost:8123")
    }
}
