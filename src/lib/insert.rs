use crate::error::ProtonClientError;
use crate::{alias::Result, ProtonClient};
use clickhouse::{insert, Row};

impl ProtonClient {
    /// Inserts a new data item into Proton.
    ///
    /// Pass in the data to insert to add it to Proton.
    ///
    /// # Arguments
    ///
    /// * `item` - The data item to insert
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// - The API call fails
    ///
    /// # Example
    /// ```no_run
    ///  use proton::ProtonClient;
    ///  use proton::prelude::{Result, Row};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Row, Serialize, Deserialize)]
    /// pub struct MyRow<'a> {
    ///     no: u32,
    ///     name: &'a str,
    /// }
    ///
    ///  async fn example() -> Result<()> {
    ///
    ///     let client = ProtonClient::new("http://localhost:8080");
    ///     let mut insert = client.insert("table_name").await?;
    ///
    ///    insert
    ///     .write(&MyRow::new(42, "foo"))
    ///     .await
    ///     .expect(" Failed to insert row into table some");
    ///
    ///     insert.end().await.expect("Failed to end insert");
    ///
    ///     Ok(())
    ///  }
    ///
    ///
    pub async fn insert<T: Row>(&self, table: &str) -> Result<insert::Insert<T>> {
        return match self.client.insert(table) {
            Ok(insert) => Ok(insert),
            Err(err) => Err(ProtonClientError::InsertFailed(err.to_string())),
        };
    }
}
