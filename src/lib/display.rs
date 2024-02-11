use std::fmt::{Display, Formatter};
use crate::ProtonClient;

impl Display for ProtonClient {
    /// Formats the ProtonClient for display.
    ///
    /// This converts the ProtonClient into a string containing the base URL.
    ///
    /// # Example
    ///
    /// ```
    /// use proton::ProtonClient;
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    ///
    /// println!("Client connected to: {}", client);
    /// ```
    ///
    /// Output:
    ///
    /// ```text
    /// Client connected to: http://localhost:8123
    /// ```
    ///
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
