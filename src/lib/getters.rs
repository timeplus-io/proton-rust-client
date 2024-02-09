use clickhouse::Client;
use crate::ProtonClient;

impl ProtonClient {
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}
