use crate::error::ProtonClientError;
use crate::{alias, ProtonClient};

impl ProtonClient {
    /// Executes a query against Proton.
    ///
    /// Pass a query string to run against the Proton data. Returns the result.
    ///
    /// # Arguments
    ///
    /// * `query` - The query string to execute
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API call fails
    /// - Query syntax is invalid
    ///
    /// # Example
    ///
    /// ```no_run
    ///  use proton_client::ProtonClient;
    ///  use proton_client::prelude::Result;
    ///
    ///  async fn example() -> Result<()> {
    ///
    ///  let client = ProtonClient::new("http://localhost:8123");
    ///
    ///      client
    ///         .execute_query("CREATE STREAM IF NOT EXISTS test_stream(no uint32, name string) ORDER BY no")
    ///         .await
    ///         .expect("Failed to execute query");
    ///
    ///   Ok(())
    /// }
    pub async fn execute_query(&self, query: &str) -> alias::Result<()> {
        return match self.client.query(query).execute().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtonClientError::QueryFailed(e.to_string())),
        };
    }
}
