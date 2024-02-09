use std::result;
use crate::error::ProtonClientError;

pub type Result<T, E = ProtonClientError> = result::Result<T, E>;
