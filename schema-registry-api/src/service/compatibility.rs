use crate::{CompatibilityResult, RegisterSchema, SchemaRegistryError, SchemaVersion, SubjectName};

use super::SchemaRegistry;

/// The compatibility client
#[derive(Debug)]
pub struct CompatibilityClient<'sr> {
    pub(super) sr: &'sr SchemaRegistry,
}

impl CompatibilityClient<'_> {
    /// Test input schema against a particular version of a subject’s schema for compatibility.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn check_version(
        &self,
        subject: &SubjectName,
        version: SchemaVersion,
        schema: RegisterSchema,
        verbose: Option<bool>,
    ) -> Result<CompatibilityResult, SchemaRegistryError> {
        let path = format!("compatibility/subjects/{subject}/versions/{version}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(verbose) = verbose {
            let query = format!("verbose={verbose}");
            url.set_query(Some(&query));
        }
        self.sr.post(url, &schema).await
    }

    /// Perform a compatibility check on the schema against one or more versions in the subject,
    /// depending on how the compatibility is set.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn check_versions(
        &self,
        subject: &SubjectName,
        schema: RegisterSchema,
        verbose: Option<bool>,
    ) -> Result<CompatibilityResult, SchemaRegistryError> {
        let path = format!("compatibility/subjects/{subject}/versions");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(verbose) = verbose {
            let query = format!("verbose={verbose}");
            url.set_query(Some(&query));
        }
        self.sr.post(url, &schema).await
    }
}
