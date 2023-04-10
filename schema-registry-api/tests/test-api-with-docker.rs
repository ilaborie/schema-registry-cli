use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

mod schema_registry;

#[tokio::test]
async fn should_works() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt()
        .pretty()
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(Level::INFO)
        .try_init();

    let port = 8081; // TODO test karapace
    schema_registry::should_works(port).await?;

    Ok(())
}
