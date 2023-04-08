use crate::{Schema, SchemaId, SchemaRegistryError, SchemaType, SubjectName, SubjectVersion};

use super::SchemaRegistry;

/// The subject client
#[derive(Debug)]
pub struct SchemaClient<'sr> {
    pub(super) sr: &'sr SchemaRegistry,
}

impl SchemaClient<'_> {
    /// Get the schema string identified by the input ID.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get(
        &self,
        id: SchemaId,
        subject: Option<&SubjectName>,
    ) -> Result<Option<Schema>, SchemaRegistryError> {
        let path = format!("schemas/ids/{id}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(subject) = subject {
            let query = format!("subject={subject}");
            url.set_query(Some(&query));
        }
        self.sr.get_optional(url).await
    }

    /// Retrieves only the schema identified by the input ID.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get_schema(
        &self,
        id: SchemaId,
        subject: Option<SubjectName>,
    ) -> Result<Option<String>, SchemaRegistryError> {
        let path = format!("schemas/ids/{id}/schema");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(subject) = subject {
            let query = format!("subject={subject}");
            url.set_query(Some(&query));
        }
        self.sr.get_optional(url).await
    }

    /// Get the schema types that are registered with Schema Registry.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn types(&self) -> Result<Vec<SchemaType>, SchemaRegistryError> {
        let url = self.sr.base_url.join("schemas/types")?;
        self.sr.get(url).await
    }

    /// Get the subject-version pairs identified by the input ID.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn versions(&self, id: SchemaId) -> Result<Vec<SubjectVersion>, SchemaRegistryError> {
        let path = format!("schemas/ids/{id}/versions");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get(url).await
    }
}
