use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::{CliError, Result, Verbosity};

/// Initialise tracing
///
/// # Errors
///
/// Fail if the tracing have already been globally configured
pub fn init_tracing(verbosity: Verbosity) -> Result<()> {
    let filter = LevelFilter::from(verbosity);
    tracing_subscriber::fmt()
        .pretty()
        .with_line_number(true)
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(filter)
        .try_init()
        .map_err(|err| CliError::InitTracingError(err.to_string()))
}

impl From<Verbosity> for LevelFilter {
    fn from(value: Verbosity) -> Self {
        match i16::from(value) {
            i16::MIN..=-2 => LevelFilter::OFF,
            -1 => LevelFilter::ERROR,
            0 => LevelFilter::WARN,
            1 => LevelFilter::INFO,
            2 => LevelFilter::DEBUG,
            3.. => LevelFilter::TRACE,
        }
    }
}
