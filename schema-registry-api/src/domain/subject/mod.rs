mod name;

use crate::{SchemaId, SchemaReference, SchemaType, SchemaVersion};

pub use self::name::*;

/// A subject an associated version
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubjectVersion {
    /// Name of the subject
    pub subject: SubjectName,
    /// Version of the returned schema
    pub version: SchemaVersion,
}

/// A subject
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    /// Name of the subject
    pub subject: SubjectName,
    /// Id of the schema
    pub id: SchemaId,
    /// Version of the schema
    pub version: SchemaVersion,
    /// The schema type
    #[serde(rename = "schema_type", default)]
    pub schema_type: SchemaType,
    /// The schema
    pub schema: String,
}

/// Register a schema
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct RegisterSchema {
    /// The schema string
    pub schema: String,
    /// The schema type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaType>,
    /// The schema references
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<SchemaReference>,
}

/// Registered schema result
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegisteredSchema {
    /// The schema id
    pub id: SchemaId,
}
