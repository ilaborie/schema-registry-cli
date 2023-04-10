use crate::{
    RegisterSchema, RegisteredSchema, SchemaId, SchemaRegistryError, SchemaVersion, Subject,
    SubjectName,
};

use super::SchemaRegistry;

/// The subject client
#[derive(Debug)]
pub struct SubjectClient<'sr> {
    pub(super) sr: &'sr SchemaRegistry,
}

impl SubjectClient<'_> {
    /// A list of all subjects.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn list(
        &self,
        subject_prefix: Option<&str>,
        deleted: Option<bool>,
    ) -> Result<Vec<SubjectName>, SchemaRegistryError> {
        let mut url = self.sr.base_url.join("subjects")?;
        if let Some(subject_prefix) = subject_prefix {
            url.query_pairs_mut()
                .append_pair("subjectPrefix", subject_prefix);
        }
        if let Some(deleted) = deleted {
            url.query_pairs_mut()
                .append_pair("deleted", deleted.to_string().as_str());
        }
        self.sr.get(url).await
    }

    /// Get a list of versions registered under the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn versions(
        &self,
        name: &SubjectName,
    ) -> Result<Vec<SchemaVersion>, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get(url).await
    }

    /// Deletes the specified subject and its associated compatibility level if registered.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn delete(
        &self,
        name: &SubjectName,
        permanent: Option<bool>,
    ) -> Result<Vec<SchemaVersion>, SchemaRegistryError> {
        let path = format!("subjects/{name}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(permanent) = permanent {
            let query = format!("permanent={permanent}");
            url.set_query(Some(&query));
        }

        self.sr.delete(url).await
    }

    /// Get a specific version of the schema registered under this subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn version(
        &self,
        name: &SubjectName,
        version: SchemaVersion,
    ) -> Result<Option<Subject>, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions/{version}");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get_optional(url).await
    }

    /// Get the schema for the specified version of this subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn schema(
        &self,
        name: &SubjectName,
        version: SchemaVersion,
    ) -> Result<Option<String>, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions/{version}/schema");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get_optional_string(url).await
    }

    /// Register a new schema under the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn new_version(
        &self,
        name: &SubjectName,
        schema: &RegisterSchema,
        normalize: Option<bool>,
    ) -> Result<RegisteredSchema, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(normalize) = normalize {
            let query = format!("normalize={normalize}");
            url.set_query(Some(&query));
        }
        self.sr.post(url, schema).await
    }

    /// Check if a schema has already been registered under the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn check_schema(
        &self,
        name: &SubjectName,
        schema: &RegisterSchema,
        normalize: Option<bool>,
    ) -> Result<Subject, SchemaRegistryError> {
        let path = format!("subjects/{name}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(normalize) = normalize {
            let query = format!("normalize={normalize}");
            url.set_query(Some(&query));
        }
        self.sr.post(url, schema).await
    }

    /// Deletes a specific version of the schema registered under this subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn delete_version(
        &self,
        name: &SubjectName,
        version: SchemaVersion,
        permanent: Option<bool>,
    ) -> Result<Option<SchemaVersion>, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions/{version}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(permanent) = permanent {
            let query = format!("permanent={permanent}");
            url.set_query(Some(&query));
        }

        self.sr.delete_option(url).await
    }

    /// Get a list of IDs of schemas that reference the schema with the given subject and version.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn referenced_by(
        &self,
        name: &SubjectName,
        version: SchemaVersion,
    ) -> Result<Vec<SchemaId>, SchemaRegistryError> {
        let path = format!("subjects/{name}/versions/{version}/referencedby");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get(url).await
    }
}
