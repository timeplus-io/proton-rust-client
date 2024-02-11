mod alias;
mod default;
mod display;
mod error;
mod fetch;
mod getters;
mod insert;
pub mod prelude;
mod query;

use clickhouse::Client;

#[derive(Clone)]
pub struct ProtonClient {
    client: Client,
    url: String,
}

impl ProtonClient {
    /// Creates a new Proton client instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL of the Proton API endpoint.
    ///
    /// # Returns
    ///
    /// Returns a new Proton client instance with the provided base URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton::ProtonClient;
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    /// ```
    pub fn new(url: &str) -> Self {
        let client = Client::default().with_url(url.to_string());
        let url = url.to_string();
        Self { client, url }
    }
}
