use reqwest::Url;
use schema_registry_api::SchemaRegistry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a `SchemaRegistry`
    let base_url = Url::parse("http://localhost:8081")?;
    let sr = SchemaRegistry::build_default(base_url)?;

    // List subjects
    let subjects = sr.subject().list(None, None).await?;

    if subjects.is_empty() {
        println!("No subject found");
    }
    for subject in &subjects {
        println!("Found subject '{subject}'");
    }

    Ok(())
}
