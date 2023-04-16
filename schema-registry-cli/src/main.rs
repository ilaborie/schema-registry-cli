use clap::Parser;
use schema_registry_cli::{init_tracing, process, Result, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::parse();
    init_tracing(settings.verbosity)?;
    process(settings).await?;
    Ok(())
}
