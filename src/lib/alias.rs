use std::result;
use crate::error::ProtonClientError;

/// Alias for Result type with predefined ProtonClientError error type
pub type Result<T, E = ProtonClientError> = result::Result<T, E>;
