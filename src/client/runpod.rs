use std::fmt;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::RunpodConfig;
use crate::Result;
use crate::service::{
    BillingService, EndpointsService, PodsService, RegistryService, TemplatesService,
    VolumesService,
};

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
/// The client provides access to these services:
/// - [`pods()`](Self::pods) - Pod lifecycle management
/// - [`endpoints()`](Self::endpoints) - Serverless endpoint operations
/// - [`templates()`](Self::templates) - Template creation and management
/// - [`volumes()`](Self::volumes) - Network volume operations
/// - [`container_registry_auth()`](Self::container_registry_auth) - Registry authentication
/// - [`billing()`](Self::billing) - Usage and billing information
///
/// # Examples
///
/// ## Basic usage with environment configuration
///
/// ```no_run
/// use runpod_sdk::{RunpodConfig, model::ListPodsQuery};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = RunpodConfig::from_env()?.build_client()?;
///
/// // List all pods
/// let pods = client.pods().list(ListPodsQuery::default()).await?;
/// println!("Found {} pods", pods.len());
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration with builder pattern
///
/// ```no_run
/// use runpod_sdk::RunpodConfig;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = RunpodConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.runpod.io/v1")
///     .with_timeout(Duration::from_secs(30))
///     .build_client()?;
///
/// // Use different services
/// let pods = client.pods().list(Default::default()).await?;
/// let endpoints = client.endpoints().list(Default::default()).await?;
/// let templates = client.templates().list(Default::default()).await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Multi-threaded usage
///
/// ```no_run
/// use runpod_sdk::RunpodConfig;
/// use std::sync::Arc;
/// use tokio::task;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Arc::new(RunpodConfig::from_env()?.build_client()?);
///
/// let handles: Vec<_> = (0..3).map(|i| {
///     let client = Arc::clone(&client);
///     task::spawn(async move {
///         let pods = client.pods().list(Default::default()).await?;
///         println!("Thread {}: Found {} pods", i, pods.len());
///         Ok::<(), runpod_sdk::Error>(())
///     })
/// }).collect();
///
/// for handle in handles {
///     handle.await??;
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

    /// Returns the pods service for Pod lifecycle operations.
    ///
    /// Provides access to Pod management including creation, listing, updates,
    /// start/stop operations, and resource monitoring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let pods_service = client.pods();
    /// let all_pods = pods_service.list(Default::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pods(&self) -> PodsService {
        PodsService::new(self.clone())
    }

    /// Returns the endpoints service for Serverless endpoint management.
    ///
    /// Provides access to serverless endpoint operations including creation,
    /// scaling configuration, deployment management, and monitoring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let endpoints_service = client.endpoints();
    /// let endpoints = endpoints_service.list(Default::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn endpoints(&self) -> EndpointsService {
        EndpointsService::new(self.clone())
    }

    /// Returns the templates service for template management.
    ///
    /// Provides access to template operations including creation, listing,
    /// updates, and template-based Pod/endpoint deployment.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let templates_service = client.templates();
    /// let templates = templates_service.list(Default::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn templates(&self) -> TemplatesService {
        TemplatesService::new(self.clone())
    }

    /// Returns the volumes service for network volume operations.
    ///
    /// Provides access to persistent storage management including volume
    /// creation, listing, updates, and attachment to Pods.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let volumes_service = client.volumes();
    /// let volumes = volumes_service.list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn volumes(&self) -> VolumesService {
        VolumesService::new(self.clone())
    }

    /// Returns the container registry authentication service.
    ///
    /// Provides access to container registry credential management for
    /// accessing private Docker images during Pod and endpoint deployment.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let registry_service = client.container_registry_auth();
    /// let auth_records = registry_service.list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn container_registry_auth(&self) -> RegistryService {
        RegistryService::new(self.clone())
    }

    /// Returns the billing service for usage and cost information.
    ///
    /// Provides access to billing data including usage statistics,
    /// cost breakdowns, and billing history for all resources.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RunpodConfig::from_env()?.build_client()?;
    /// let billing_service = client.billing();
    /// let pod_billing = billing_service.pods(Default::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn billing(&self) -> BillingService {
        BillingService::new(self.clone())
    }

    /// Creates a GET request.
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .get(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a POST request.
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .post(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a PUT request.
    #[allow(dead_code)]
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .put(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a PATCH request.
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .patch(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }

    /// Creates a DELETE request.
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .delete(&url)
            .bearer_auth(self.inner.config.api_key())
            .timeout(self.inner.config.timeout())
    }
}

impl fmt::Debug for RunpodClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunpodClient")
            .field("api_key", &self.inner.config.masked_api_key())
            .field("base_url", &self.inner.config.base_url())
            .field("timeout", &self.inner.config.timeout())
            .finish()
    }
}
