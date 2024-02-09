use clickhouse::query::RowCursor;
use clickhouse::Row;
use serde::Deserialize;
use crate::{alias::Result, ProtonClient, error::ProtonClientError};

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
    /// let mut cursor = ProtonClient::default()
    ///     .query("SELECT ?fields FROM some WHERE no BETWEEN 0 AND 1")
    ///     .fetch::<MyRow<'_>>()?;
    ///
    /// while let Some(MyRow { name, no }) = cursor.next().await? {
    ///     println!("{name}: {no}");
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn fetch<T: Row>(&self, query: &str) -> Result<RowCursor<T>> {
        return match self.client
            .query(query)
            .fetch::<T>() {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchFailed(e.to_string())),
        };
    }

    ///Executes the query and returns all the generated results, collected into a Vec.
    /// Note that T must be owned.
    pub async fn fetch_all<T: Row>(&self, query: &str) -> Result<Vec<T>>
        where
            T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client
            .query(query)
            .fetch_all::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchAllFailed(e.to_string())),
        };
    }

    /// Executes the query and returns just a single row.
    ///
    /// Note that `T` must be owned.
    pub async fn fetch_one<T>(self, query: &str) -> Result<T>
        where
            T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client
            .query(query)
            .fetch_one::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchOneFailed(e.to_string())),
        };
    }

    /// Executes the query and returns at most one row.
    ///
    /// Note that `T` must be owned.
    pub async fn fetch_optional<T>(self, query: &str) -> Result<Option<T>>
        where
            T: Row + for<'b> Deserialize<'b>,
    {
        return match self.client
            .query(query)
            .fetch_optional::<T>().await {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchOptionalFailed(e.to_string())),
        };
    }
}
