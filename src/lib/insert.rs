use crate::error::ProtonClientError;
use crate::{alias::Result, ProtonClient};
use clickhouse::{insert, inserter, Row};

impl ProtonClient {
    /// Inserts a new data item into Proton.
    ///
    /// Pass in the data to insert to add it to Proton.
    ///
    /// # Arguments
    ///
    /// * `table` - The table name to insert into
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
    ///  use serde::{Deserialize, Serialize};
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

impl ProtonClient {
    /// Inserts bulk data into Proton.
    ///
    /// # Arguments
    ///
    /// * `table` - The table name to insert into
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
    ///  use serde::{Deserialize, Serialize};
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
    ///      let mut inserter = client
    ///         .inserter("table_name")?
    ///         .with_max_rows(100_000); //  The maximum number of rows in one INSERT statement.
    ///
    ///     for i in 0..1000 {
    ///         inserter.write(&MyRow { no: i, name: "foo" })?;
    ///         inserter.commit().await?; // Checks limits and ends a current INSERT if they are reached.
    ///     }
    ///
    ///     inserter.end().await?; // Ends a current INSERT and whole Inserter unconditionally.
    ///
    ///     Ok(())
    ///  }
    ///
    ///
    pub async fn inserter<T: Row>(&self, table: &str) -> Result<inserter::Inserter<T>> {
        return match self.client.inserter(table) {
            Ok(inserter) => Ok(inserter),
            Err(err) => Err(ProtonClientError::InserterFailed(err.to_string())),
        };
    }
}
