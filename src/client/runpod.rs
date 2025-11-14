//! RunPod API client implementation.
//!
//! This module contains the main [`RunpodClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the RunPod API.

use std::fmt;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::RunpodConfig;
use crate::Result;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;

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
///
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
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::model::ListPodsQuery;
/// use runpod_sdk::service::PodsService;
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
///     .with_rest_url("https://rest.runpod.io/v1")
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
/// The client is cheap to clone (uses `Arc` internally):
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::service::PodsService;
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
pub struct RunpodClient {
    pub(crate) inner: Arc<RunpodClientInner>,
}

/// Inner client state that is shared via Arc for cheap cloning.
#[derive(Debug)]
pub(crate) struct RunpodClientInner {
    pub(crate) config: RunpodConfig,
    pub(crate) client: Client,
}

impl RunpodClient {
    /// Creates a new Runpod API client.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip(config),
            target = TRACING_TARGET_CLIENT,
            fields(api_key = %config.masked_api_key())
        )
    )]
    pub fn new(config: RunpodConfig) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating RunPod client");

        let client = Client::builder().timeout(config.timeout()).build()?;

        #[cfg(feature = "tracing")]
        tracing::info!(target: TRACING_TARGET_CLIENT,
            rest_url = %config.rest_url(),
            timeout = ?config.timeout(),
            api_key = %config.masked_api_key(),
            "RunPod client created successfully"
        );

        let inner = Arc::new(RunpodClientInner { config, client });
        Ok(Self { inner })
    }

    /// Makes a GET request to the API endpoint URL (not GraphQL).
    ///
    /// This is a low-level method for making GET requests to the RunPod API.
    /// The path should be relative to the API base URL (e.g., "endpoint_id/status/job_id").
    #[cfg(feature = "endpoint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "endpoint")))]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "GET", path, url)
        )
    )]
    pub(crate) fn get_api(&self, path: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.inner.config.api_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "GET",
            "Creating HTTP GET request to API"
        );

        self.inner
            .client
            .get(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Makes a POST request to the API endpoint URL (not GraphQL).
    ///
    /// This is a low-level method for making POST requests to the RunPod API.
    /// The path should be relative to the API base URL (e.g., "endpoint_id/run").
    #[cfg(feature = "endpoint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "endpoint")))]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "POST", path, url)
        )
    )]
    pub(crate) fn post_api(&self, path: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.inner.config.api_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "POST",
            "Creating HTTP POST request to API"
        );

        self.inner
            .client
            .post(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a GET request.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "GET", path, url)
        )
    )]
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.rest_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "GET",
            "Creating HTTP GET request"
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
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "POST", path, url)
        )
    )]
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.rest_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "POST",
            "Creating HTTP POST request"
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
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "PATCH", path, url)
        )
    )]
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.rest_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "PATCH",
            "Creating HTTP PATCH request"
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
        tracing::instrument(
            skip(self),
            target = TRACING_TARGET_CLIENT,
            fields(method = "DELETE", path, url)
        )
    )]
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.rest_url(), path);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: TRACING_TARGET_CLIENT,
            url = %url,
            method = "DELETE",
            "Creating HTTP DELETE request"
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
    /// let client = RunpodClient::from_env()?;
    /// let query = r#"{ viewer { id name } }"#;
    /// let response: MyResponse = client.graphql_query(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            skip(self, query),
            target = TRACING_TARGET_CLIENT,
            fields(query_len = query.len(), url, status)
        )
    )]
    pub async fn graphql_query<T>(&self, query: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = self.inner.config.graphql_url();

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT,
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
        tracing::debug!(target: TRACING_TARGET_CLIENT,
            status = %status,
            success = status.is_success(),
            "GraphQL response received"
        );

        let result = response.json().await?;
        Ok(result)
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
    #[cfg_attr(feature = "tracing", tracing::instrument(target = TRACING_TARGET_CLIENT))]
    pub fn from_env() -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating RunPod client from environment");

        let config = RunpodConfig::from_env()?;
        Self::new(config)
    }
}

impl fmt::Debug for RunpodClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("RunpodClient");
        debug_struct
            .field("api_key", &self.inner.config.masked_api_key())
            .field("rest_url", &self.inner.config.rest_url())
            .field("timeout", &self.inner.config.timeout());

        #[cfg(feature = "graphql")]
        debug_struct.field("graphql_url", &self.inner.config.graphql_url());

        debug_struct.finish()
    }
}
