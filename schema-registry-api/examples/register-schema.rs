use reqwest::Url;
use schema_registry_api::{RegisterSchema, SchemaRegistry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a `SchemaRegistry`
    let base_url = Url::parse("http://localhost:8081")?;
    let sr = SchemaRegistry::build_default(base_url)?;

    // Create a subject
    let subject = "a-topic-value".parse()?;
    // Create the `RegisterSchema`
    let schema = RegisterSchema {
        schema: include_str!("../tests/assets/a_record.avsc").to_string(),
        ..Default::default()
    };

    // Register the schema for the subject
    let registered_schema = sr.subject().new_version(&subject, &schema, None).await?;
    // Get the schema id
    let schema_id = registered_schema.id;

    println!("The schema #{schema_id} is registered for subject '{subject}'");

    Ok(())
}
