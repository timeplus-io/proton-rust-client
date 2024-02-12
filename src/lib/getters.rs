use crate::ProtonClient;
use clickhouse::Client;

impl ProtonClient {
    /// Returns a reference to the internal clickhouse client.
    ///
    /// This can be used for making lower-level requests.
    ///
    /// For details on how to use the client, see the [clickhouse client crate](https://github.com/loyd/clickhouse.rs).
    ///
    /// # Example
    /// ```no_run
    /// use proton_client::prelude::{ProtonClient, Result};
    ///
    /// async fn example() -> Result<()> {
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    ///
    /// let url = client.client().await;
    ///
    /// Ok(())
    /// }
    /// ```
    ///
    pub async fn client(&self) -> &Client {
        &self.client
    }

    /// Returns the base URL of the Proton API endpoint.
    ///
    /// This is the root URL passed to the client during construction.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton_client::prelude::{ProtonClient, Result};
    ///
    /// async fn example() -> Result<()> {
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    ///
    /// let url = client.url().await;
    ///
    /// assert_eq!(url, "http://localhost:8123");
    ///
    /// Ok(())
    /// }
    /// ```
    ///
    pub async fn url(&self) -> &str {
        &self.url
    }
}
