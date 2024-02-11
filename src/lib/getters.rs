use clickhouse::Client;
use crate::ProtonClient;

impl ProtonClient {
    /// Returns a reference to the internal clickhouse client.
    ///
    /// This can be used for making lower-level requests.
    ///
    /// For details on how to use the client, see the [clickhouse client crate](https://github.com/loyd/clickhouse.rs).
    ///
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns the base URL of the Proton API endpoint.
    ///
    /// This is the root URL passed to the client during construction.
    ///
    /// # Example
    ///
    /// ```
    /// use proton::ProtonClient;
    ///
    /// let client = ProtonClient::new("https://api.proton.com");
    ///
    /// let url = client.url();
    ///
    /// assert_eq!(url, "https://api.proton.com");
    /// ```
    ///
    pub fn url(&self) -> &str {
        &self.url
    }
}
