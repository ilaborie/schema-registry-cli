use assert2::{check, let_assert};
use schema_registry_api::{SchemaClient, SchemaRegistryError, SchemaType, Subject};
use tracing::info;

pub async fn should_works_no_schema<'a>(client: &SchemaClient<'a>) -> anyhow::Result<()> {
    let id = "42".parse()?;

    info!("schemaGet");
    let result = client.get(id, None).await?;
    let_assert!(None = result);

    info!("schemaGetSchema");
    let result = client.get_schema(id, None).await?;
    let_assert!(None = result);

    info!("schemaTypes");
    let mut result = client.types().await?;
    result.sort_by_key(|t| t.to_string());
    check!(result == &[SchemaType::Avro, SchemaType::Json, SchemaType::Protobuf]);

    info!("schemaVersions");
    let error = client.versions(id).await.unwrap_err();
    let_assert!(SchemaRegistryError::ApiError(err) = error);
    check!(err.error_code == 40403);

    Ok(())
}

pub async fn should_works_with_schema<'a>(
    client: &SchemaClient<'a>,
    subject: &Subject,
) -> anyhow::Result<()> {
    let id = subject.id;
    let name = subject.subject.clone();
    let version = subject.version;
    let schema_normalized = subject.schema.replace([' ', '\n'], "");

    info!("schemaGet");
    let result = client.get(id, None).await?;
    let_assert!(Some(sc) = result);
    check!(sc.schema == schema_normalized);

    info!("schemaGet2");
    let result = client.get(id, Some(&name)).await?;
    let_assert!(Some(sc) = result);
    check!(sc.schema == schema_normalized);

    // TODO it's not working?
    // info!("schemaGetSchema");
    // let result = client.get_schema(id, None).await?;
    // let_assert!(Some(sc) = result);
    // check!(sc == schema_normalized);

    info!("schemaVersions");
    let result = client.versions(id).await?;
    let sv = result.iter().find(|it| it.subject == name);
    let_assert!(Some(sv) = sv);
    check!(sv.version == version);

    Ok(())
}
