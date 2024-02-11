use crate::error::ProtonClientError;
use std::result;

/// Alias for Result type with predefined ProtonClientError error type
pub type Result<T, E = ProtonClientError> = result::Result<T, E>;
