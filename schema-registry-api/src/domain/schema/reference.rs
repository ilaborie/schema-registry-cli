use crate::SchemaVersion;

/// A schema reference
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaReference {
    /// A name for the reference.
    ///
    /// For Avro, the reference name is the fully qualified schema name,
    /// for JSON Schema it is a URL,
    /// and for Protobuf, it is the name of another Protobuf file.
    pub name: String,

    /// A subject
    ///
    /// representing the subject under which the referenced schema is registered.
    pub subject: String,

    /// Version of the schema
    pub version: SchemaVersion,
}
