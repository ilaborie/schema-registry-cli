use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use reqwest::{Client, Response, StatusCode, Url};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_tracing::TracingMiddleware;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod compatibility;
mod config;
mod error;
mod mode;
mod schema;
mod subject;

use crate::ApiError;

pub use self::compatibility::*;
pub use self::config::*;
pub use self::error::*;
pub use self::mode::*;
pub use self::schema::*;
pub use self::subject::*;

/// A schema registry client
#[derive(Debug, Clone)]
pub struct SchemaRegistry {
    base_url: Url,
    client: ClientWithMiddleware,
}

impl SchemaRegistry {
    /// Create a schema registry client
    #[must_use]
    pub fn new(base_url: Url, client: ClientWithMiddleware) -> Self {
        Self { base_url, client }
    }

    /// Create a schema registry client
    ///
    /// # Errors
    ///
    /// Fail if we cannot build the reqwest client
    pub fn build_default(base_url: Url) -> Result<Self, SchemaRegistryError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.schemaregistry.v1+json"),
        );
        let reqwest_client = Client::builder().default_headers(headers).build()?;
        let client = reqwest_middleware::ClientBuilder::new(reqwest_client)
            // Insert the tracing middleware
            .with(TracingMiddleware::default())
            .build();
        Ok(Self::new(base_url, client))
    }

    async fn get<B>(&self, url: Url) -> Result<B, SchemaRegistryError>
    where
        B: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        handle_response(response).await
    }

    async fn get_optional<B>(&self, url: Url) -> Result<Option<B>, SchemaRegistryError>
    where
        B: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        handle_optional_response(response).await
    }

    async fn get_optional_string(&self, url: Url) -> Result<Option<String>, SchemaRegistryError> {
        let response = self.client.get(url).send().await?;
        handle_optional_string_response(response).await
    }

    async fn post<R, B>(&self, url: Url, body: &R) -> Result<B, SchemaRegistryError>
    where
        R: Serialize,
        B: DeserializeOwned,
    {
        let response = self.client.post(url).json(body).send().await?;
        handle_response(response).await
    }

    async fn put<R, B>(&self, url: Url, body: &R) -> Result<B, SchemaRegistryError>
    where
        R: Serialize,
        B: DeserializeOwned,
    {
        let response = self.client.put(url).json(body).send().await?;
        handle_response(response).await
    }

    async fn delete<B>(&self, url: Url) -> Result<B, SchemaRegistryError>
    where
        B: DeserializeOwned,
    {
        let response = self.client.delete(url).send().await?;
        handle_response(response).await
    }

    async fn delete_option<B>(&self, url: Url) -> Result<Option<B>, SchemaRegistryError>
    where
        B: DeserializeOwned,
    {
        let response = self.client.delete(url).send().await?;
        handle_optional_response(response).await
    }

    /// Schema client
    #[must_use]
    pub fn schema(&self) -> SchemaClient {
        SchemaClient { sr: self }
    }

    /// Subject client
    #[must_use]
    pub fn subject(&self) -> SubjectClient {
        SubjectClient { sr: self }
    }

    /// Mode client
    #[must_use]
    pub fn mode(&self) -> ModeClient {
        ModeClient { sr: self }
    }

    /// Compatibility client
    #[must_use]
    pub fn compatibility(&self) -> CompatibilityClient {
        CompatibilityClient { sr: self }
    }

    /// Configuration client
    #[must_use]
    pub fn config(&self) -> ConfigClient {
        ConfigClient { sr: self }
    }
}

async fn handle_response<T>(response: Response) -> Result<T, SchemaRegistryError>
where
    T: DeserializeOwned,
{
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let err = handle_error(response).await;
        Err(err)
    }
}

async fn handle_optional_response<T>(response: Response) -> Result<Option<T>, SchemaRegistryError>
where
    T: DeserializeOwned,
{
    let status = response.status();
    if status.is_success() {
        let result = response.json().await?;
        Ok(Some(result))
    } else if status == StatusCode::NOT_FOUND || status == StatusCode::NO_CONTENT {
        Ok(None)
    } else {
        let err = handle_error(response).await;
        Err(err)
    }
}

async fn handle_optional_string_response(
    response: Response,
) -> Result<Option<String>, SchemaRegistryError> {
    let status = response.status();
    if status.is_success() {
        let result = response.text().await?;
        Ok(Some(result))
    } else if status == StatusCode::NOT_FOUND || status == StatusCode::NO_CONTENT {
        Ok(None)
    } else {
        let err = handle_error(response).await;
        Err(err)
    }
}

async fn handle_error(response: Response) -> SchemaRegistryError {
    let body = response.text().await.unwrap_or_default();
    if let Ok(error) = serde_json::from_str::<ApiError>(&body) {
        SchemaRegistryError::ApiError(error)
    } else {
        SchemaRegistryError::SchemaRegistryError(body)
    }
}
