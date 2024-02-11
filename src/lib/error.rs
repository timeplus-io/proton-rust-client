use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// An error returned by the Proton Client API itself.
#[derive(Debug)]
pub enum ProtonClientError {
    /// An error occurred during a query operation.
    ///
    /// This variant wraps the error message.
    QueryFailed(String),

    /// An error occurred during an insert operation.
    ///
    /// This variant wraps the error message.
    InsertFailed(String),

    /// An error occurred during a bulk insert operation.
    ///
    /// This variant wraps the error message.
    InserterFailed(String),

    /// An error occurred while fetching data.
    ///
    /// This variant wraps the error message.
    FetchFailed(String),

    /// An error occurred while fetching all data.
    ///
    /// This variant wraps the error message.
    FetchAllFailed(String),

    /// An error occurred while fetching a single data item.
    ///
    /// This variant wraps the error message.
    FetchOneFailed(String),

    /// An error occurred while fetching optional data.
    ///
    /// This variant wraps the error message.
    FetchOptionalFailed(String),
}

impl Error for ProtonClientError {}

impl Display for ProtonClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProtonClientError::QueryFailed(msg) => {
                write!(f, "[ProtonClient]: Query failed with error: {}", msg)
            }
            ProtonClientError::InsertFailed(msg) => {
                write!(f, "[ProtonClient]: Insert failed with error: {}", msg)
            }
            ProtonClientError::InserterFailed(msg) => {
                write!(f, "[ProtonClient]: Inserter failed with error: {}", msg)
            }
            ProtonClientError::FetchFailed(msg) => {
                write!(f, "[ProtonClient]: Fetch failed with error: {}", msg)
            }
            ProtonClientError::FetchAllFailed(msg) => {
                write!(f, "[ProtonClient]: FetchAll failed with error: {}", msg)
            }
            ProtonClientError::FetchOneFailed(msg) => {
                write!(f, "[ProtonClient]: FetchOne failed with error: {}", msg)
            }
            ProtonClientError::FetchOptionalFailed(msg) => {
                write!(f, "[ProtonClient]: FetchOption failed with error: {}", msg)
            }
        }
    }
}
