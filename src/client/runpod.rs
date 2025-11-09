//! RunPod API client implementation.
//!
//! This module contains the main [`RunpodClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the RunPod API.

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::RunpodConfig;
use super::version::{ApiVersion, V1};
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
/// The client implements V1 API service traits that provide direct access to API methods:
/// - [`PodsService`](crate::service::v1::PodsService) - Pod lifecycle management
/// - [`EndpointsService`](crate::service::v1::EndpointsService) - Serverless endpoint operations
/// - [`TemplatesService`](crate::service::v1::TemplatesService) - Template creation and management
/// - [`VolumesService`](crate::service::v1::VolumesService) - Network volume operations
/// - [`RegistryService`](crate::service::v1::RegistryService) - Registry authentication
/// - [`BillingService`](crate::service::v1::BillingService) - Usage and billing information
///
/// # Examples
///
/// ## Basic usage with environment configuration
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::model::v1::ListPodsQuery;
/// use runpod_sdk::service::v1::PodsService;
///
/// # async fn example() -> Result<()> {
/// let client: RunpodClient = RunpodClient::from_env()?;
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
/// use runpod_sdk::service::v1::{PodsService, EndpointsService, TemplatesService};
/// use std::time::Duration;
///
/// # async fn example() -> Result<()> {
/// let client = RunpodConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.runpod.io/v1")
///     .with_timeout(Duration::from_secs(30))
///     .build_v1()?;
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
/// The client is cheap to clone (uses `Arc` internally):
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::service::v1::PodsService;
/// use tokio::task;
///
/// # async fn example() -> Result<()> {
/// let client = RunpodClient::from_env()?;
///
/// let handles: Vec<_> = (0..3).map(|i| {
///     let client = client.clone();
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
pub struct RunpodClient<V: ApiVersion = V1> {
    inner: Arc<RunpodClientInner>,
    _version: PhantomData<V>,
}

/// Inner client state that is shared via Arc for cheap cloning.
#[derive(Debug)]
struct RunpodClientInner {
    config: RunpodConfig,
    client: Client,
}

impl<V: ApiVersion> RunpodClient<V> {
    /// Creates a new Runpod API client.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config), fields(api_key = %config.masked_api_key())))]
    pub fn new(config: RunpodConfig) -> Result<Self> {
        let client = Client::builder().timeout(config.timeout()).build()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            base_url = %config.base_url(),
            timeout = ?config.timeout(),
            api_key = %config.masked_api_key(),
            "Created Runpod client"
        );

        let inner = Arc::new(RunpodClientInner { config, client });
        Ok(Self {
            inner,
            _version: PhantomData,
        })
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
    /// let client: RunpodClient = RunpodClient::from_env()?;
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
        tracing::instrument(skip(self), fields(method = "GET", path, url))
    )]
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(
            url = %url,
            method = "GET",
            "Creating HTTP request"
        );

        self.inner
            .client
            .get(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a POST request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "POST", path, url))
    )]
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(
            url = %url,
            method = "POST",
            "Creating HTTP request"
        );

        self.inner
            .client
            .post(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a PATCH request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "PATCH", path, url))
    )]
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(
            url = %url,
            method = "PATCH",
            "Creating HTTP request"
        );

        self.inner
            .client
            .patch(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a DELETE request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip(self), fields(method = "DELETE", path, url))
    )]
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);

        #[cfg(feature = "tracing")]
        tracing::debug!(
            url = %url,
            method = "DELETE",
            "Creating HTTP request"
        );

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
    /// let client: RunpodClient = RunpodClient::from_env()?;
    /// let query = r#"{ viewer { id name } }"#;
    /// let response: MyResponse = client.graphql_query(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, query), fields(query_len = query.len(), url, status)))]
    pub async fn graphql_query<T>(&self, query: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = self.inner.config.graphql_url();

        #[cfg(feature = "tracing")]
        tracing::debug!(
            url = %url,
            query_len = query.len(),
            api_key = %self.inner.config.masked_api_key(),
            "Executing GraphQL query"
        );

        let request = self
            .inner
            .client
            .post(url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
            .json(&serde_json::json!({ "query": query }));

        let response = request.send().await?;
        let status = response.status();

        #[cfg(feature = "tracing")]
        tracing::debug!(
            status = %status,
            success = status.is_success(),
            "GraphQL response received"
        );

        let result = response.json().await?;
        Ok(result)
    }
}

impl<V: ApiVersion> fmt::Debug for RunpodClient<V> {
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
