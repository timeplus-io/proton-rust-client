use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProtonClientError {
    QueryFailed(String),
    InsertFailed(String),
    FetchFailed(String),
    FetchAllFailed(String),
    FetchOneFailed(String),
    FetchOptionalFailed(String),
}

impl Error for ProtonClientError {}

impl Display for ProtonClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProtonClientError::QueryFailed(msg) => write!(f, "[ProtonClient]: Query failed with error: {}", msg),
            ProtonClientError::InsertFailed(msg) => write!(f, "[ProtonClient]: Insert failed with error: {}", msg),
            ProtonClientError::FetchFailed(msg) => write!(f, "[ProtonClient]: Fetch failed with error: {}", msg),
            ProtonClientError::FetchAllFailed(msg) => write!(f, "[ProtonClient]: FetchAll failed with error: {}", msg),
            ProtonClientError::FetchOneFailed(msg) => write!(f, "[ProtonClient]: FetchOne failed with error: {}", msg),
            ProtonClientError::FetchOptionalFailed(msg) => write!(f, "[ProtonClient]: FetchOption failed with error: {}", msg),
        }
    }
}

