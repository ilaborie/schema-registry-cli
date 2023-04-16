#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::perf)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

/// The result type
pub type Result<T> = std::result::Result<T, CliError>;

mod error;
mod init_tracing;
mod settings;

/// Commands that can be run
pub mod command;

pub use self::error::*;
pub use self::init_tracing::*;
pub use self::settings::*;

/// Process the command
///
/// # Errors
///
/// Fail if the command fail
pub async fn process(settings: Settings) -> Result<()> {
    let verbose = settings.verbosity >= Verbosity::INFO;
    match settings.command {
        Command::Subject(sub_cmd) => match sub_cmd {
            SubjectSubCommand::List(l) => command::display_list_subjects(l).await?,
            SubjectSubCommand::Register(r) => command::display_register_schema(r).await?,
            SubjectSubCommand::Delete(d) => command::display_delete_subject(d).await?,
            SubjectSubCommand::Check(c) => command::display_check_compatibility(c, verbose).await?,
        },
        Command::Schema(sub_cmd) => match sub_cmd {
            SchemaSubCommand::Get(g) => command::display_get_schema(g).await?,
        },
        Command::Completion(c) => command::generate_completion(c)?,
    }

    Ok(())
}
