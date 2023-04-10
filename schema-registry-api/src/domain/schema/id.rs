use std::fmt::Display;
use std::str::FromStr;

use crate::SchemaIdError;

/// The schema id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SchemaId(u64);

impl FromStr for SchemaId {
    type Err = SchemaIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<u64>().map_err(|_| SchemaIdError(s.to_string()))?;
        Ok(Self(id))
    }
}

impl Display for SchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
