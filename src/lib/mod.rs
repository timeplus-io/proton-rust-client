mod alias;
mod default;
mod display;
mod error;
mod fetch;
mod getters;
mod insert;
pub mod prelude;
mod query;
mod stream;

use clickhouse::{Client, Compression};

#[derive(Clone)]
pub struct ProtonClient {
    client: Client,
    // A separate client without compression is currently necessary to enable
    // streaming queries in release builds. See issue #6 on Github:
    // https://github.com/timeplus-io/proton-rust-client/issues/6
    client_without_compression: Client,
    url: String,
}

impl ProtonClient {
    /// Creates a new Proton client instance.
    ///`
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
    /// use proton_client::ProtonClient;
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    /// ```
    pub fn new(url: &str) -> Self {
        let client = Client::default().with_url(url.to_string());
        let client_without_compression =
            Client::with_compression(client.clone(), Compression::None);
        let url = url.to_string();
        Self {
            client,
            client_without_compression,
            url,
        }
    }
}
