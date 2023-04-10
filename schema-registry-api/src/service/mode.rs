use crate::{Mode, SchemaRegistryError, SubjectName};

use super::SchemaRegistry;

/// The subject client
#[derive(Debug)]
pub struct ModeClient<'sr> {
    pub(super) sr: &'sr SchemaRegistry,
}

impl ModeClient<'_> {
    /// Get the current mode for Schema Registry at a global level.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get(&self) -> Result<Mode, SchemaRegistryError> {
        let url = self.sr.base_url.join("mode")?;
        self.sr.get(url).await
    }

    /// Update the global Schema Registry mode.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn set(&self, mode: Mode, force: Option<bool>) -> Result<Mode, SchemaRegistryError> {
        let mut url = self.sr.base_url.join("mode")?;
        if let Some(force) = force {
            let query = format!("force={force}");
            url.set_query(Some(&query));
        }
        self.sr.put(url, &mode).await
    }

    /// Get the mode for a subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn get_subject(&self, subject: &SubjectName) -> Result<Mode, SchemaRegistryError> {
        let path = format!("mode/{subject}");
        let url = self.sr.base_url.join(&path)?;
        self.sr.get(url).await
    }

    /// Update the mode for the specified subject.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn set_subject(
        &self,
        subject: &SubjectName,
        mode: Mode,
        force: Option<bool>,
    ) -> Result<Mode, SchemaRegistryError> {
        let path = format!("mode/{subject}");
        let mut url = self.sr.base_url.join(&path)?;
        if let Some(force) = force {
            let query = format!("force={force}");
            url.set_query(Some(&query));
        }
        self.sr.put(url, &mode).await
    }

    /// Deletes the subject-level mode for the specified subject and reverts to the global default.
    ///
    /// # Errors
    ///
    /// Fail if we cannot send the query
    /// Fail if the schema registry return an error
    #[tracing::instrument(skip(self))]
    pub async fn delete_subject(&self, subject: &SubjectName) -> Result<Mode, SchemaRegistryError> {
        let path = format!("mode/{subject}");
        let url = self.sr.base_url.join(&path)?;
        self.sr.delete(url).await
    }
}
