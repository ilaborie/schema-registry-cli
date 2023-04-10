use assert2::{check, let_assert};
use schema_registry_api::{
    RegisterSchema, RegisteredSchema, SchemaRegistryError, SchemaType, Subject, SubjectClient,
};
use tracing::info;

use crate::schema_registry::{given_a_test_subject, AN_AVRO_SCHEMA};

pub async fn should_works_no_subject<'a>(client: &SubjectClient<'a>) -> anyhow::Result<()> {
    info!("subjectList");
    let result = client.list(Some("real-"), None).await?;
    check!(result == &[]);

    let name = "not_exists".parse()?;
    let version = "latest".parse()?;

    info!("subjectVersions");
    let error = client.versions(&name).await.unwrap_err();
    let_assert!(SchemaRegistryError::ApiError(err) = error);
    check!(err.error_code == 40401);

    info!("subjectDelete");
    let error = client.delete(&name, None).await.unwrap_err();
    let_assert!(SchemaRegistryError::ApiError(err) = error);
    check!(err.error_code == 40401);

    info!("subjectVersion");
    let result = client.version(&name, version).await?;
    check!(result == None);

    info!("subjectSchema");
    let result = client.schema(&name, version).await?;
    check!(result == None);

    info!("subjectDeleteVersion");
    let result = client.delete_version(&name, version, None).await?;
    check!(result == None);

    info!("subjectReferencedBy");
    let error = client.referenced_by(&name, version).await.unwrap_err();
    let_assert!(SchemaRegistryError::ApiError(err) = error);
    check!(err.error_code == 40401);

    let schema = RegisterSchema {
        schema: AN_AVRO_SCHEMA.to_string(),
        schema_type: None,
        references: vec![],
    };

    info!("subjectCheckSchema");
    let error = client.check_schema(&name, &schema, None).await.unwrap_err();
    let_assert!(SchemaRegistryError::ApiError(err) = error);
    check!(err.error_code == 40401);

    Ok(())
}

pub async fn should_works_with_subject_delete<'a>(
    client: &SubjectClient<'a>,
) -> anyhow::Result<()> {
    let name = given_a_test_subject();
    let schema = RegisterSchema {
        schema: AN_AVRO_SCHEMA.to_string(),
        schema_type: None,
        references: vec![],
    };

    info!("subjectNewVersion");
    let result = client.new_version(&name, &schema, Some(false)).await;
    let_assert!(Ok(_) = &result);

    info!("subjectVersions");
    let versions = client.versions(&name).await?;
    check!(versions.len() == 1);
    let_assert!(Some(version) = versions.into_iter().next());

    info!("subjectDeleteVersion");
    let result = client.delete_version(&name, version, Some(false)).await?;
    let_assert!(Some(sv) = result);
    check!(sv == version);

    info!("subjectDelete");
    let result = client.delete(&name, Some(true)).await?;
    check!(result == &[version]);

    Ok(())
}

pub async fn should_works_with_subject<'a>(client: &SubjectClient<'a>) -> anyhow::Result<Subject> {
    let name = given_a_test_subject();
    let schema = RegisterSchema {
        schema: AN_AVRO_SCHEMA.to_string(),
        schema_type: None,
        references: vec![],
    };
    let schema_normalized = AN_AVRO_SCHEMA.replace(' ', "").replace('\n', "");

    info!("subjectNewVersion");
    let result = client.new_version(&name, &schema, None).await;
    let_assert!(Ok(RegisteredSchema { id }) = &result);

    info!("subjectList");
    let result = client.list(Some("test-"), None).await?;
    check!(result.contains(&name));

    info!("subjectVersions");
    let versions = client.versions(&name).await?;
    check!(versions.len() == 1);
    let_assert!(Some(version) = versions.into_iter().next());

    info!("subjectVersion");
    let result = client.version(&name, version).await?;
    let_assert!(Some(s) = result);
    check!(
        s == Subject {
            subject: name.clone(),
            id: id.clone(),
            version,
            schema_type: SchemaType::Avro,
            schema: schema_normalized.clone(),
        }
    );

    info!("subjectSchema");
    let result = client.schema(&name, version).await?;
    let_assert!(Some(sc) = result);
    check!(sc == schema_normalized);

    info!("subjectReferencedBy");
    let result = client.referenced_by(&name, version).await?;
    check!(result == &[]);

    info!("subjectCheckSchema");
    let result = client.check_schema(&name, &schema, None).await?;
    check!(
        result
            == Subject {
                subject: name.clone(),
                id: id.clone(),
                version,
                schema_type: SchemaType::Avro,
                schema: schema_normalized,
            }
    );

    Ok(s)
}
