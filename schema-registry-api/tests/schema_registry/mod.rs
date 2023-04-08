use nanoid::nanoid;
use schema_registry_api::{SchemaRegistry, SubjectName};

pub mod compatibility_client;
pub mod config_client;
pub mod mode_client;
pub mod schema_client;
pub mod subject_client;

const AN_AVRO_SCHEMA: &str = include_str!("../assets/a_record.avsc");

pub async fn should_works(port: u16) -> anyhow::Result<()> {
    let base_url = format!("http://localhost:{port}");
    let base_url = base_url.parse()?;

    let sr = SchemaRegistry::build_default(base_url)?;

    let subject = sr.subject();
    subject_client::should_works_no_subject(&subject).await?;
    subject_client::should_works_with_subject_delete(&subject).await?;
    let subject = subject_client::should_works_with_subject(&subject).await?;

    let schema = sr.schema();
    schema_client::should_works_no_schema(&schema).await?;
    schema_client::should_works_with_schema(&schema, &subject).await?;

    let mode = sr.mode();
    mode_client::should_works_no_subject(&mode).await?;
    mode_client::should_works_subject(&mode, &subject.subject).await?;

    let config = sr.config();
    config_client::should_works_no_subject(&config).await?;
    config_client::should_works_subject(&config, &subject.subject).await?;

    let compatibility = sr.compatibility();
    compatibility_client::should_works_no_subject(&compatibility).await?;
    compatibility_client::should_works_subject(&compatibility, &subject).await?;

    Ok(())
}

fn given_a_test_subject() -> SubjectName {
    let id = nanoid!();
    let name = format!("test-{id}");
    name.parse().expect("Should be valid")
}
