use crate::{Compatibility, GetCompatibility, SchemaRegistryError, SubjectName};

use super::SchemaRegistry;

/// The configuration client
#[derive(Debug)]
pub struct ConfigClient<'sr> {
    pub(super) sr: &'sr SchemaRegistry,
}

impl ConfigClient<'_> {
    /// Update global compatibility level.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn set(
        &self,
        compatibility: Compatibility,
    ) -> Result<Compatibility, SchemaRegistryError> {
        let url = self.sr.base_url.join("config")?;
        self.sr.put(url, &compatibility).await
    }

    /// Get global compatibility level.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get(&self) -> Result<GetCompatibility, SchemaRegistryError> {
        let url = self.sr.base_url.join("config")?;
        self.sr.get(url).await
    }

    /// Update compatibility level for the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn set_subject(
        &self,
        subject: &SubjectName,
        compatibility: Compatibility,
    ) -> Result<Compatibility, SchemaRegistryError> {
        let path = format!("config/{subject}");
        let url = self.sr.base_url.join(&path)?;
        self.sr.put(url, &compatibility).await
    }

    /// Update compatibility level for the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get_subject(
        &self,
        subject: &SubjectName,
        default_to_global: Option<bool>,
    ) -> Result<GetCompatibility, SchemaRegistryError> {
        let path = format!("config/{subject}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(default_to_global) = default_to_global {
            let query = format!("defaultToGlobal={default_to_global}");
            url.set_query(Some(&query));
        }
        self.sr.get(url).await
    }

    /// Deletes the specified subject-level compatibility level config and reverts to the global default.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn delete_subject(
        &self,
        subject: &SubjectName,
    ) -> Result<GetCompatibility, SchemaRegistryError> {
        let path = format!("config/{subject}");
        let url = self.sr.base_url.join(&path)?;
        self.sr.delete(url).await
    }
}
