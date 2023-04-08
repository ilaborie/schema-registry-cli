use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use crate::SubjectNameError;

/// A subject name
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SubjectName(String);

impl AsRef<str> for SubjectName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for SubjectName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for SubjectName {
    type Err = SubjectNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::EmptyName);
        }
        if s.chars().any(char::is_control) {
            return Err(Self::Err::InvalidChar(s.to_string()));
        }
        Ok(Self(s.to_string()))
    }
}
impl From<String> for SubjectName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for SubjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// FIXME test parse
