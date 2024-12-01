use std::fmt::Display;
use std::num::NonZeroU32;
use std::str::FromStr;

use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use crate::SchemaVersionError;

/// The schema version
///
/// A schema version could be a number in [1, 2^31-1],
/// or the 'latest' version.
///
/// You can build a schema version from a string
///
/// ```rust
/// #  use schema_registry_api::SchemaVersion;
/// let version = "2".parse::<SchemaVersion>().unwrap();
/// let latest1 = "latest".parse::<SchemaVersion>().unwrap();
/// let latest2 = "-1".parse::<SchemaVersion>().unwrap();
/// assert_eq!(latest1, latest2);
/// ```
///
/// Note that version could not start with `0`
///
/// ```rust
/// #  use schema_registry_api::SchemaVersion;
/// let result = "0".parse::<SchemaVersion>(); // 🚨 Error
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchemaVersion {
    /// A standard version
    ///
    /// Should be in [1,2^31-1]
    Version(NonZeroU32),
    /// The latest version
    Latest,
}

impl FromStr for SchemaVersion {
    type Err = SchemaVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == LATEST_STRING || s == "-1" {
            return Ok(Self::Latest);
        }
        let number = s
            .parse::<NonZeroU32>()
            .map_err(|_| SchemaVersionError(s.to_string()))?;
        Ok(Self::Version(number))
    }
}

const LATEST_STRING: &str = "latest";

impl Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Version(v) => write!(f, "{v}"),
            Self::Latest => write!(f, "{LATEST_STRING}"),
        }
    }
}

impl Serialize for SchemaVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SchemaVersion::Version(v) => serializer.serialize_u32(v.get()),
            SchemaVersion::Latest => serializer.serialize_str(LATEST_STRING),
        }
    }
}

struct SchemaVersionVisitor;
impl Visitor<'_> for SchemaVersionVisitor {
    type Value = SchemaVersion;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected an u32 version number of the \"latest\" string")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let i = u32::try_from(v).map_err(serde::de::Error::custom)?;
        let version = NonZeroU32::try_from(i).map_err(serde::de::Error::custom)?;
        Ok(Self::Value::Version(version))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == -1 {
            Ok(Self::Value::Latest)
        } else {
            let msg = format!("Invalid negative value, only support -1 as 'latest', got {v}");
            Err(serde::de::Error::custom(msg))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == LATEST_STRING {
            Ok(Self::Value::Latest)
        } else {
            let msg = format!("Expected 'latest', got {v}");
            Err(serde::de::Error::custom(msg))
        }
    }
}

impl<'de> Deserialize<'de> for SchemaVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SchemaVersionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn should_serde_version() {
        let json = "42";
        let version = serde_json::from_str::<SchemaVersion>(json).unwrap();
        check!(version == SchemaVersion::Version(NonZeroU32::new(42).unwrap()));
        let s = serde_json::to_string(&version).unwrap();
        check!(s == json);
    }

    #[test]
    fn should_serde_latest_version() {
        let json = "\"latest\"";
        let version = serde_json::from_str::<SchemaVersion>(json).unwrap();
        check!(version == SchemaVersion::Latest);
        let s = serde_json::to_string(&version).unwrap();
        check!(s == json);
    }

    #[test]
    fn should_serde_latest_version_minus_one() {
        let json = "-1";
        let version = serde_json::from_str::<SchemaVersion>(json).unwrap();
        check!(version == SchemaVersion::Latest);
    }

    #[rstest::rstest]
    #[case::bad_string("plop")]
    #[case::zero("0")]
    #[case::negative("-2")]
    #[case::too_big("4294967296")]
    fn should_not_serde(#[case] json: &str) {
        let result = serde_json::from_str::<SchemaVersion>(json);
        let_assert!(Err(_) = result);
    }
}
