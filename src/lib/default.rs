use crate::ProtonClient;

impl Default for ProtonClient {
    /// Returns a default configured Proton client instance.
    ///
    /// The default configuration includes:
    ///
    /// - Base URL set to the Proton production API endpoint
    ///
    /// # Returns
    ///
    /// A Proton client instance with default configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use proton::ProtonClient;
    ///
    /// let proton = ProtonClient::default();
    /// ```
    fn default() -> Self {
        Self::new("http://localhost:8123")
    }
}
