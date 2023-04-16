use std::path::Path;

use schema_registry_api::{
    CompatibilityResult, SchemaId, SchemaRegistry, SchemaVersion, Subject, SubjectName,
};

use crate::{
    CheckCompatibility, DeleteSubject, ListSubjects, RegisterSchemaSettings, Result,
    SchemaRegistrySettings,
};

use super::build_register_schema;

/// List subjects
///
/// # Errors
///
/// Fail if the API fail
pub async fn list_subjects(
    client_settings: SchemaRegistrySettings,
    all: bool,
    prefix: Option<&str>,
) -> Result<Vec<Subject>> {
    let client = SchemaRegistry::try_from(client_settings)?;
    let names = client.subject().list(prefix, Some(all)).await?;
    let mut result = Vec::with_capacity(names.len());
    for name in &names {
        let subject = client
            .subject()
            .version(name, SchemaVersion::Latest)
            .await?;
        if let Some(subject) = subject {
            result.push(subject);
        }
    }
    Ok(result)
}

pub(crate) async fn display_list_subjects(list_subjects: ListSubjects) -> Result<()> {
    let ListSubjects {
        schema_registry,
        all,
        prefix,
    } = list_subjects;
    let subjects = self::list_subjects(schema_registry, all, prefix.as_deref()).await?;

    // Display
    if subjects.is_empty() {
        println!("No subject founds");
    } else {
        println!("Found {} subjects", subjects.len());
        for subject in &subjects {
            let Subject {
                subject,
                id,
                version,
                schema_type,
                ..
            } = subject;
            println!("Subject '{subject}', schema [{schema_type}] #{id} v{version}");
        }
    }

    Ok(())
}

/// Check schema compatibility for a subject
///
/// # Errors
///
/// Fail if the API fail
pub async fn check_compatibility(
    client_settings: SchemaRegistrySettings,
    subject: &SubjectName,
    path: &Path,
    version: Option<SchemaVersion>,
    verbose: bool,
) -> Result<CompatibilityResult> {
    let client = SchemaRegistry::try_from(client_settings)?;
    let schema = build_register_schema(path)?;
    let result = if let Some(version) = version {
        client
            .compatibility()
            .check_version(subject, version, &schema, Some(verbose))
            .await?
    } else {
        client
            .compatibility()
            .check_versions(subject, &schema, Some(verbose))
            .await?
    };
    Ok(result)
}

pub(crate) async fn display_check_compatibility(
    check_compatibility: CheckCompatibility,
    verbose: bool,
) -> Result<()> {
    let CheckCompatibility {
        schema_registry,
        subject,
        version,
        path,
    } = check_compatibility;

    let result =
        self::check_compatibility(schema_registry, &subject, &path, version, verbose).await?;

    // Display
    if result.is_compatible {
        println!("✅ the schema is compatible");
    } else {
        println!("❌ the schema is NOT compatible");
    }

    Ok(())
}

/// Register a schema for a subject
///
/// # Errors
///
/// Fail if the API fail
pub async fn register_schema(
    client_settings: SchemaRegistrySettings,
    subject: &SubjectName,
    path: &Path,
    normalize: bool,
) -> Result<SchemaId> {
    let client = SchemaRegistry::try_from(client_settings)?;
    let schema = build_register_schema(path)?;
    let result = client
        .subject()
        .new_version(subject, &schema, Some(normalize))
        .await?;
    Ok(result.id)
}

pub(crate) async fn display_register_schema(register_schema: RegisterSchemaSettings) -> Result<()> {
    let RegisterSchemaSettings {
        schema_registry,
        subject,
        normalize,
        path,
    } = register_schema;

    let result = self::register_schema(schema_registry, &subject, &path, normalize).await?;

    // Display
    println!("Registered schema id: {result}");

    Ok(())
}

/// Delete subject
///
/// # Errors
///
/// Fail if the API fail
pub async fn delete_subject(
    client_settings: SchemaRegistrySettings,
    subject: &SubjectName,
    version: Option<SchemaVersion>,
    permanent: bool,
) -> Result<Vec<SchemaVersion>> {
    let client = SchemaRegistry::try_from(client_settings)?;
    let result = if let Some(version) = version {
        client
            .subject()
            .delete_version(subject, version, Some(permanent))
            .await?
            .into_iter()
            .collect()
    } else {
        client.subject().delete(subject, Some(permanent)).await?
    };
    Ok(result)
}

pub(crate) async fn display_delete_subject(list_subjects: DeleteSubject) -> Result<()> {
    let DeleteSubject {
        schema_registry,
        subject,
        version,
        permanent,
    } = list_subjects;
    let versions = self::delete_subject(schema_registry, &subject, version, permanent).await?;

    // Display
    if versions.is_empty() {
        println!("No subject version deleted");
    } else {
        println!("Delete {} versions", versions.len());
        for version in &versions {
            println!("{version}");
        }
    }

    Ok(())
}
