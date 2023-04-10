use assert2::check;
use schema_registry_api::{CompatibilityClient, RegisterSchema, SchemaVersion, Subject};
use tracing::info;

use crate::schema_registry::AN_AVRO_SCHEMA;

pub async fn should_works_no_subject<'a>(client: &CompatibilityClient<'a>) -> anyhow::Result<()> {
    let name = "not-exists".parse()?;
    let version = SchemaVersion::Latest;
    let schema = RegisterSchema {
        schema: AN_AVRO_SCHEMA.to_string(),
        schema_type: None,
        references: vec![],
    };

    info!("compatibilityCheckVersion");
    let result = client
        .check_version(&name, version, schema.clone(), None)
        .await?;
    check!(result.is_compatible == true);

    info!("compatibilityCheckVersions");
    let result = client.check_versions(&name, schema, None).await?;
    check!(result.is_compatible == true);

    Ok(())
}

pub async fn should_works_subject<'a>(
    client: &CompatibilityClient<'a>,
    subject: &Subject,
) -> anyhow::Result<()> {
    let name = &subject.subject;
    let version = subject.version;
    let schema = RegisterSchema {
        schema: AN_AVRO_SCHEMA.to_string(),
        schema_type: None,
        references: vec![],
    };

    info!("compatibilityCheckVersion");
    let result = client
        .check_version(name, version, schema.clone(), None)
        .await?;
    check!(result.is_compatible == true);

    info!("compatibilityCheckVersions");
    let result = client.check_versions(name, schema, None).await?;
    check!(result.is_compatible == true);

    Ok(())
}
