use std::fmt::Display;
use std::str::FromStr;

use crate::SchemaIdError;

/// The schema id
///
/// You can build the schema id from a string or from an `u64`
///
/// ```rust
/// # use schema_registry_api::SchemaId;
/// let id = "1".parse::<SchemaId>().expect("Should be a valid id");
/// let id2 = SchemaId::from(1);
/// assert_eq!(id, id2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SchemaId(u64);

impl FromStr for SchemaId {
    type Err = SchemaIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<u64>().map_err(|_| SchemaIdError(s.to_string()))?;
        Ok(Self(id))
    }
}

impl From<u64> for SchemaId {
    fn from(value: u64) -> Self {
        SchemaId(value)
    }
}

impl Display for SchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn should_build_from_str() {
        let str = "42";
        let result = SchemaId::from_str(str).unwrap();
        check!(result == SchemaId(42));
    }

    #[test]
    fn should_not_build_from_invalid_str() {
        let str = "a42";
        let result = SchemaId::from_str(str);
        let_assert!(Err(_) = result);
    }
}
