use clickhouse::{insert, Row};
use crate::{alias::Result, ProtonClient};
use crate::error::ProtonClientError;

impl ProtonClient {
    pub async fn insert<T: Row>(&self, table: &str) -> Result<insert::Insert<T>> {
        return match self.client
            .insert(table) {
            Ok(insert) => Ok(insert),
            Err(err) => Err(ProtonClientError::InsertFailed(err.to_string())),
        };
    }
}
