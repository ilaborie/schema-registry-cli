use std::str::FromStr;

use clap::{Args, Parser, Subcommand};
use url::Url;

mod completion;
mod schema;
mod subjects;
mod verbosity;

pub use self::completion::*;
pub use self::schema::*;
pub use self::subjects::*;
pub use self::verbosity::*;

/// Schema Registry CLI arguments
#[derive(Debug, Clone, Parser)]
#[clap(version, author = env!("CARGO_PKG_HOMEPAGE"), about)]
pub struct Settings {
    /// Verbosity
    #[clap(flatten)]
    pub verbosity: Verbosity,

    /// Command
    #[clap(subcommand)]
    pub command: Command,
}

/// Available commands
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Subject commands
    #[clap(subcommand)]
    Subject(SubjectSubCommand),

    /// Schema commands
    #[clap(subcommand)]
    Schema(SchemaSubCommand),

    /// Generate shell completions
    Completion(Completion),
}

const DEFAULT_SCHEMA_REGISTRY_URL: &str = "http://localhost:8081";

/// Schema registry settings
#[derive(Debug, Clone, PartialEq, Args)]
pub struct SchemaRegistrySettings {
    /// The schema registry URL
    #[clap(short, long, default_value = DEFAULT_SCHEMA_REGISTRY_URL)]
    pub url: Url,
}

impl Default for SchemaRegistrySettings {
    fn default() -> Self {
        let url = Url::from_str(DEFAULT_SCHEMA_REGISTRY_URL).expect("Valid URL");
        Self { url }
    }
}
