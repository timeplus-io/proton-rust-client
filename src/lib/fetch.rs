use crate::{alias::Result, error::ProtonClientError, ProtonClient};
use clickhouse::query::RowCursor;
use clickhouse::Row;
use serde::Deserialize;

impl ProtonClient {
    /// Executes the query, returning a [`RowCursor`] to obtain results.
    ///
    /// # Example
    ///
    /// ```no_run
    ///  use proton::ProtonClient;
    ///  use proton::prelude::Result;
    ///
    ///  async fn example() -> Result<()> {
    ///
    /// #[derive(clickhouse::Row, serde::Deserialize)]
    /// struct MyRow<'a> {
    ///     no: u32,
    ///     name: &'a str,
    /// }
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    ///
    /// let mut cursor = client
    ///     .query("SELECT ?fields FROM some WHERE no BETWEEN 0 AND 1")
    ///     .fetch::<MyRow<'_>>()?;
    ///
    /// while let Some(MyRow { name, no }) = cursor.next().await? {
    ///     println!("{name}: {no}");
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn fetch<T: Row>(&self, query: &str) -> Result<RowCursor<T>> {
        return match self.client.query(query).fetch::<T>() {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchFailed(e.to_string())),
        };
    }

    /// Executes the query and returns all the generated results, collected into a Vec.
    /// Note that T must be owned.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The API call fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton::prelude::{ProtonClient, Result};
    ///
    /// async fn example() -> Result<()> {
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    ///
    /// let query = "SELECT ?fields FROM some WHERE no BETWEEN 0 AND 1";
    /// let data = client.fetch_all(query).unwrap();
    ///
    /// println!("Received {} records", data.len());
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_all<T: Row>(&self, query: &str) -> Result<Vec<T>>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client.query(query).fetch_all::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchAllFailed(e.to_string())),
        };
    }

    /// Executes the query and returns just a single row.
    ///
    /// Note that `T` must be owned.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The API call fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton::prelude::{ProtonClient, Result};
    ///
    /// async fn example() -> Result<()> {
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    /// let query = "select count() from table(table_name)";
    /// let item = client.fetch_one(query).unwrap();
    ///
    /// println!("Single result: {:#?}", item);
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_one<T>(self, query: &str) -> Result<T>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client.query(query).fetch_one::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchOneFailed(e.to_string())),
        };
    }

    /// Executes the query and returns at most one row.
    ///
    /// Note that `T` must be owned.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The API call fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use proton::prelude::{ProtonClient, Result};
    ///
    /// async fn example() -> Result<()> {
    ///
    /// let client = ProtonClient::new("http://localhost:8123");
    /// let item_id = 42;
    /// let query = "SELECT ?fields FROM some WHERE no = 42";
    /// let item = client.fetch_optional(query).unwrap();
    ///
    /// match item {
    ///     Some(item) => println!("Fetched: {:#?}", item),
    ///     None => println!("No item with id {}", item_id),
    /// }
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_optional<T>(self, query: &str) -> Result<Option<T>>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client.query(query).fetch_optional::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchOptionalFailed(e.to_string())),
        };
    }
}
