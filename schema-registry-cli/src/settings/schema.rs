use clap::{Args, Subcommand};
use schema_registry_api::{SchemaId, SubjectName};

use crate::SchemaRegistrySettings;

/// Subject commands
#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum SchemaSubCommand {
    /// List subjects
    Get(GetSchema),
    // TODO types, versions
}

/// Get a schema by id
#[derive(Debug, Clone, PartialEq, Args)]
pub struct GetSchema {
    /// The schema registry
    #[clap(flatten)]
    pub schema_registry: SchemaRegistrySettings,

    /// A subject name
    #[clap(short, long)]
    pub subject: Option<SubjectName>,

    /// The schema id
    pub id: SchemaId,
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use clap::Parser;

    use crate::settings::DEFAULT_SCHEMA_REGISTRY_URL;

    use super::*;

    #[derive(Debug, Parser)]
    struct JustSchema {
        #[clap(subcommand)]
        command: SchemaSubCommand,
    }

    #[rstest::rstest]
    #[case(&["bin", "get", "42"], SchemaSubCommand::Get(GetSchema { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: None,
        id: SchemaId::from(42),
    }))]
    #[case(&["bin", "get", "--subject", "plop", "42"], SchemaSubCommand::Get(GetSchema { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: Some("plop".parse().unwrap()), 
        id: SchemaId::from(42),
    }))]
    #[case(&["bin", "get", "42", "-s", "plop"], SchemaSubCommand::Get(GetSchema { 
        schema_registry: SchemaRegistrySettings {
            url: DEFAULT_SCHEMA_REGISTRY_URL.parse().unwrap(),
        },
        subject: Some("plop".parse().unwrap()), 
        id: SchemaId::from(42),
    }))]
    fn should_parse_schema(#[case] args: &[&str], #[case] expected: SchemaSubCommand) {
        let result = JustSchema::parse_from(args);
        check!(result.command == expected);
    }
}
