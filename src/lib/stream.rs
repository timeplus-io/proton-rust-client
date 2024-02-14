use crate::prelude::{ProtonClient, ProtonClientError, Result};
use clickhouse::query::RowCursor;
use clickhouse::Row;
use serde::Deserialize;

impl ProtonClient {
    ///
    /// Executes a streaming query, returning a [`RowCursor`] to obtain results
    /// as they become available from the stream. The key difference compared to fetch is that,
    /// for streaming query, the returned result is a unbounded stream. Also,
    /// a fetch_stream query will keep running continuously returning fresh data
    /// until the application terminates..
    ///
    /// # Example
    ///
    /// ```no_run
    ///  use proton_client::ProtonClient;
    ///  use proton_client::prelude::Result;
    ///
    ///  async fn example() -> Result<()> {
    ///
    /// #[derive(Debug, clickhouse::Row, serde::Deserialize)]
    /// struct MyRow {
    ///     no: u32,
    ///     name: String,
    /// }
    ///
    /// let client = ProtonClient::new("http://localhost:3218");
    ///
    ///  let mut cursor = client
    ///     .fetch_stream::<MyRow>("SELECT ?fields from (test_stream) WHERE no BETWEEN 500 AND 504")
    ///     .await
    ///     .expect("[main/fetch]: Failed to fetch stream data");
    ///
    /// while let Some(MyRow { name, no }) = cursor.next().await.expect("[main/fetch]: Failed to fetch data") {
    ///     println!("{name}: {no}");
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn fetch_stream<T>(&self, query: &str) -> Result<RowCursor<T>>
    where
        T: Row + for<'b> Deserialize<'b>,
    {
        // Here we use the client without compression. For details, see:
        // https://github.com/timeplus-io/proton-rust-client/issues/6
        match self.client_without_compression.query(query).fetch::<T>() {
            Ok(cursor) => Ok(cursor),
            Err(e) => Err(ProtonClientError::FetchFailed(e.to_string())),
        }
    }
}
