// mod check_compatibility;
// mod delete_subject;
// mod get_schema;
// mod list_subjects;
// mod register_schema;

mod completion;
mod schema;
mod subject;

use std::fs;
use std::path::Path;

use schema_registry_api::{RegisterSchema, SchemaRegistry, SchemaType};

use crate::{CliError, Result, SchemaRegistrySettings};

pub(crate) use self::completion::*;
pub use self::schema::*;
pub use self::subject::*;

impl TryFrom<SchemaRegistrySettings> for SchemaRegistry {
    type Error = CliError;

    fn try_from(value: SchemaRegistrySettings) -> std::result::Result<Self, Self::Error> {
        let result = SchemaRegistry::build_default(value.url)?;
        Ok(result)
    }
}

fn build_register_schema(path: &Path) -> Result<RegisterSchema> {
    let extension = path
        .extension()
        .ok_or_else(|| CliError::InvalidSchemaExtension(path.to_path_buf()))?
        .to_string_lossy();
    let schema_type = match extension.as_ref() {
        "avsc" | "avro" => SchemaType::Avro,
        "json" => SchemaType::Json,
        "proto" => SchemaType::Protobuf,
        _ => {
            return Err(CliError::InvalidSchemaExtension(path.to_path_buf()));
        }
    };
    let schema = fs::read_to_string(path)?;

    let result = RegisterSchema {
        schema,
        schema_type: Some(schema_type),
        references: vec![],
    };
    Ok(result)
}
