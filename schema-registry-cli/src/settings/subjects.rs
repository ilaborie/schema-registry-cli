use std::path::PathBuf;

use clap::{Args, Subcommand};
use schema_registry_api::{SchemaVersion, SubjectName};

use crate::SchemaRegistrySettings;

/// Subject commands
#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum SubjectSubCommand {
    /// List subjects
    List(ListSubjects),

    /// Register schema
    Register(RegisterSchemaSettings),

    /// Check schema compatibility
    Check(CheckCompatibility),

    /// Delete subject
    Delete(DeleteSubject),
}

impl Default for SubjectSubCommand {
    fn default() -> Self {
        Self::List(ListSubjects::default())
    }
}

/// List subjects completions for shell
#[derive(Debug, Clone, PartialEq, Default, Args)]
pub struct ListSubjects {
    /// The schema registry
    #[clap(flatten)]
    pub schema_registry: SchemaRegistrySettings,

    /// Include deleted subjects
    #[clap(long)]
    pub all: bool,

    /// Filter subjects with this prefix
    pub prefix: Option<String>,
}

/// Register a schema to a subject
#[derive(Debug, Clone, PartialEq, Args)]
pub struct RegisterSchemaSettings {
    /// The schema registry
    #[clap(flatten)]
    pub schema_registry: SchemaRegistrySettings,

    /// The subject name
    #[clap(short, long)]
    pub subject: SubjectName,

    /// Normalize the schema
    #[clap(long)]
    pub normalize: bool,

    /// The schema file
    pub path: PathBuf,
}

/// Delete subject
#[derive(Debug, Clone, PartialEq, Args)]
pub struct DeleteSubject {
    /// The schema registry
    #[clap(flatten)]
    pub schema_registry: SchemaRegistrySettings,

    /// The subject name
    #[clap(short, long)]
    pub subject: SubjectName,

    /// The schema version
    #[clap(long)]
    pub version: Option<SchemaVersion>,

    /// Delete permanent
    #[clap(long)]
    pub permanent: bool,
}

/// Check schema compatibility
#[derive(Debug, Clone, PartialEq, Args)]
pub struct CheckCompatibility {
    /// The schema registry
    #[clap(flatten)]
    pub schema_registry: SchemaRegistrySettings,

    /// The subject name
    #[clap(short, long)]
    pub subject: SubjectName,

    /// The schema version
    #[clap(long)]
    pub version: Option<SchemaVersion>,

    /// The schema file
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use clap::Parser;

    use crate::settings::DEFAULT_SCHEMA_REGISTRY_URL;

    use super::*;

    #[derive(Debug, Parser)]
    struct JustSubject {
        #[clap(subcommand)]
        command: SubjectSubCommand,
    }

    #[rstest::rstest]
    #[case::list(&["bin", "list"], SubjectSubCommand::List(ListSubjects { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        all: false,
        prefix: None,
    }))]
    #[case::list(&["bin", "list", "--all"], SubjectSubCommand::List(ListSubjects { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        all: true,
        prefix: None,
    }))]
    #[case::list(&["bin", "list", "plop-"], SubjectSubCommand::List(ListSubjects { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        all: false,
        prefix: Some("plop-".to_string()),
    }))]
    #[case::register(&["bin", "register", "--subject", "plop", "./plop-value.avsc"], SubjectSubCommand::Register(RegisterSchemaSettings { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        normalize: false,
        path: PathBuf::from("./plop-value.avsc"),
    }))]
    #[case::register(&["bin", "register", "-s", "plop", "./plop-value.avsc", "--normalize"], SubjectSubCommand::Register(RegisterSchemaSettings { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        normalize: true,
        path: PathBuf::from("./plop-value.avsc"),
    }))]
    #[case::check(&["bin", "check", "--subject", "plop", "./plop-value.avsc"], SubjectSubCommand::Check(CheckCompatibility { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        version: None,
        path: PathBuf::from("./plop-value.avsc"),
    }))]
    #[case::check(&["bin", "check", "-s", "plop", "--version", "42", "./plop-value.avsc"], SubjectSubCommand::Check(CheckCompatibility { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        version: Some("42".parse().unwrap()),
        path: PathBuf::from("./plop-value.avsc"),
    }))]
    #[case::delete(&["bin", "delete", "--subject", "plop"], SubjectSubCommand::Delete(DeleteSubject { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        version: None,
        permanent: false,
    }))]
    #[case::delete(&["bin", "delete", "-s", "plop", "--version", "42", "--permanent"], SubjectSubCommand::Delete(DeleteSubject { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: "plop".parse().unwrap(), 
        version: Some("42".parse().unwrap()),
        permanent: true,
    }))]
    fn should_parse_schema(#[case] args: &[&str], #[case] expected: SubjectSubCommand) {
        let result = JustSubject::parse_from(args);
        check!(result.command == expected);
    }
}
