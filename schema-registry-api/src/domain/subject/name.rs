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

impl Display for SubjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn should_parse_subject_name() {
        let name = "plop";
        let result = name.parse::<SubjectName>();
        let_assert!(Ok(subject) = result);
        check!(subject.as_ref() == name);
        check!(subject.to_lowercase() == name);
    }

    #[test]
    fn should_not_parse_empty_subject_name() {
        let name = "";
        let result = name.parse::<SubjectName>();
        let_assert!(Err(SubjectNameError::EmptyName) = result);
    }

    #[test]
    fn should_not_parse_bad_subject_name() {
        let name = "\nasd";
        let result = name.parse::<SubjectName>();
        let_assert!(Err(SubjectNameError::InvalidChar(_)) = result);
    }
}
