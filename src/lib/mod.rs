mod display;
mod default;
mod getters;
mod alias;
pub mod prelude;
mod error;
mod fetch;
mod insert;
mod query;

use clickhouse::{Client};

#[derive(Clone)]
pub struct ProtonClient {
    client: Client,
    url: String,
}

impl ProtonClient {
    pub fn new(url: &str) -> Self {
        let client = Client::default().with_url(url.to_string());
        let url = url.to_string();
        Self { client, url }
    }
}
