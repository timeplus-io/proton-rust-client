use crate::{alias, ProtonClient};
use crate::error::ProtonClientError;

impl ProtonClient {
    pub async fn execute(&self, query: &str) -> alias::Result<()> {
        return match self.client
            .query(query)
            .execute().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtonClientError::QueryFailed(e.to_string())),
        };
    }
}
