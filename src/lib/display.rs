use std::fmt::{Display, Formatter};
use crate::ProtonClient;

impl Display for ProtonClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtonClient connected to: {}", self.url)
    }
}
