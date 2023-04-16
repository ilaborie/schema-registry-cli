use schema_registry_api::{Schema, SchemaId, SchemaRegistry, SubjectName};

use crate::{GetSchema, Result, SchemaRegistrySettings};

/// Get schema from id
///
/// # Errors
///
/// Fail if the API fail
pub async fn get_schema(
    client_settings: SchemaRegistrySettings,
    id: SchemaId,
    subject: Option<&SubjectName>,
) -> Result<Option<Schema>> {
    let client = SchemaRegistry::try_from(client_settings)?;
    let result = client.schema().get(id, subject).await?;
    Ok(result)
}

pub(crate) async fn display_get_schema(get_schema: GetSchema) -> Result<()> {
    let GetSchema {
        schema_registry,
        id,
        subject,
    } = get_schema;
    let schema = self::get_schema(schema_registry, id, subject.as_ref()).await?;

    // Display
    if let Some(schema) = schema {
        println!("{}", schema.schema);
    } else {
        println!("No schema with id {id} found");
    }

    Ok(())
}
