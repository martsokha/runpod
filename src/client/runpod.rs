use std::fmt;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::RunpodConfig;
use crate::Result;

/// Main RunPod API client for interacting with all RunPod services.
///
/// The `RunpodClient` provides access to all RunPod API endpoints through specialized
/// service interfaces. It handles authentication, request/response serialization,
/// and provides a consistent async interface for all operations.
///
/// # Features
///
/// - **Thread-safe**: Safe to use across multiple threads
/// - **Cheap to clone**: Uses `Arc` internally for efficient cloning
/// - **Automatic authentication**: Handles API key authentication automatically
/// - **Comprehensive coverage**: Access to all RunPod services (Pods, Endpoints, Templates, etc.)
///
/// # Services
///
/// The client implements service traits that provide direct access to API methods:
/// - [`PodsService`](crate::service::PodsService) - Pod lifecycle management
/// - [`EndpointsService`](crate::service::EndpointsService) - Serverless endpoint operations
/// - [`TemplatesService`](crate::service::TemplatesService) - Template creation and management
/// - [`VolumesService`](crate::service::VolumesService) - Network volume operations
/// - [`RegistryService`](crate::service::RegistryService) - Registry authentication
/// - [`BillingService`](crate::service::BillingService) - Usage and billing information
///
/// # Examples
///
/// ## Basic usage with environment configuration
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result, model::ListPodsQuery, service::PodsService};
///
/// # async fn example() -> Result<()> {
/// let client = RunpodClient::from_env()?;
///
/// // List all pods
/// let pods = client.list_pods(ListPodsQuery::default()).await?;
/// println!("Found {} pods", pods.len());
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration with builder pattern
///
/// ```no_run
/// use runpod_sdk::{RunpodConfig, RunpodClient, Result};
/// use runpod_sdk::service::{PodsService, EndpointsService, TemplatesService};
/// use std::time::Duration;
///
/// # async fn example() -> Result<()> {
/// let client = RunpodConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.runpod.io/v1")
///     .with_timeout(Duration::from_secs(30))
///     .build_client()?;
///
/// // Use different services
/// let pods = client.list_pods(Default::default()).await?;
/// let endpoints = client.list_endpoints(Default::default()).await?;
/// let templates = client.list_templates(Default::default()).await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Multi-threaded usage
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result, service::PodsService};
/// use std::sync::Arc;
/// use tokio::task;
///
/// # async fn example() -> Result<()> {
/// let client = Arc::new(RunpodClient::from_env()?);
///
/// let handles: Vec<_> = (0..3).map(|i| {
///     let client = Arc::clone(&client);
///     task::spawn(async move {
///         let pods = client.list_pods(Default::default()).await?;
///         println!("Thread {}: Found {} pods", i, pods.len());
///         Ok::<(), runpod_sdk::Error>(())
///     })
/// }).collect();
///
/// for handle in handles {
///     handle.await.unwrap()?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct RunpodClient {
    inner: Arc<RunpodClientInner>,
}

/// Inner client state that is shared via Arc for cheap cloning.
#[derive(Debug)]
struct RunpodClientInner {
    config: RunpodConfig,
    client: Client,
}

impl RunpodClient {
    /// Creates a new Runpod API client.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config)))]
    pub fn new(config: RunpodConfig) -> Result<Self> {
        let client = Client::builder().timeout(config.timeout()).build()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(base_url = %config.base_url(), timeout = ?config.timeout(), "Created Runpod client");

        let inner = Arc::new(RunpodClientInner { config, client });
        Ok(Self { inner })
    }

    /// Creates a new Runpod API client from environment variables.
    ///
    /// This is a convenience method that creates a RunpodConfig from environment
    /// variables and then creates a client from that config.
    ///
    /// # Environment Variables
    ///
    /// - `RUNPOD_API_KEY` - Your RunPod API key (required)
    /// - `RUNPOD_BASE_URL` - Base URL for the API (optional, defaults to <https://rest.runpod.io/v1>)
    /// - `RUNPOD_GRAPHQL_URL` - GraphQL API URL (optional, defaults to <https://api.runpod.io/graphql>, requires `graphql` feature)
    /// - `RUNPOD_TIMEOUT_SECS` - Request timeout in seconds (optional, defaults to 30)
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn from_env() -> Result<Self> {
        let config = RunpodConfig::from_env()?;
        Self::new(config)
    }

    /// Creates a GET request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "GET", path))
    )]
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(url = %url, "Creating GET request");

        self.inner
            .client
            .get(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a POST request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "POST", path))
    )]
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(url = %url, "Creating POST request");

        self.inner
            .client
            .post(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a PATCH request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "PATCH", path))
    )]
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(url = %url, "Creating PATCH request");

        self.inner
            .client
            .patch(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a DELETE request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "DELETE", path))
    )]
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(url = %url, "Creating DELETE request");

        self.inner
            .client
            .delete(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Executes a GraphQL query.
    ///
    /// # Arguments
    ///
    /// * `query` - The GraphQL query string
    ///
    /// # Returns
    ///
    /// Returns the deserialized response data of type `T`.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use serde::Deserialize;
    /// # #[derive(Deserialize)]
    /// # struct MyResponse {
    /// #     data: String,
    /// # }
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let query = r#"{ viewer { id name } }"#;
    /// let response: MyResponse = client.graphql_query(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, query), fields(query_len = query.len())))]
    pub async fn graphql_query<T>(&self, query: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = self.inner.config.graphql_url();

        #[cfg(feature = "tracing")]
        tracing::debug!(url = %url, "Executing GraphQL query");

        let request = self
            .inner
            .client
            .post(url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
            .json(&serde_json::json!({ "query": query }));
        let response = request.send().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(status = %response.status(), "GraphQL response received");

        let result = response.json().await?;
        Ok(result)
    }
}

impl fmt::Debug for RunpodClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("RunpodClient");
        debug_struct
            .field("api_key", &self.inner.config.masked_api_key())
            .field("base_url", &self.inner.config.base_url())
            .field("timeout", &self.inner.config.timeout());

        #[cfg(feature = "graphql")]
        debug_struct.field("graphql_url", &self.inner.config.graphql_url());

        debug_struct.finish()
    }
}
