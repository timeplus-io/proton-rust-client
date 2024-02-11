use crate::ProtonClient;

impl Default for ProtonClient {
    /// Returns a default configured Proton client instance.
    ///
    /// The default configuration includes:
    ///
    /// - Default URL is set to http://localhost:8123
    ///
    /// # Returns
    ///
    /// A Proton client instance with default configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton_client::ProtonClient;
    ///
    /// let proton = ProtonClient::default();
    /// ```
    fn default() -> Self {
        Self::new("http://localhost:8123")
    }
}
