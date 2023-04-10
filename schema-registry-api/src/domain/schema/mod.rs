mod id;
mod reference;
mod version;

use std::fmt::Display;

pub use self::id::*;
pub use self::reference::*;
pub use self::version::*;

/// A Schema type
#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SchemaType {
    /// Avro
    #[default]
    Avro,
    /// Protobuf
    Protobuf,
    /// JSON
    Json,
}

impl Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Avro => write!(f, "Avro"),
            SchemaType::Protobuf => write!(f, "Protobuf"),
            SchemaType::Json => write!(f, "Json"),
        }
    }
}

/// A Schema payload
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schema {
    /// The schema as string
    pub schema: String,
}
