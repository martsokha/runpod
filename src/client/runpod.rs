use std::fmt;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};

use super::config::RunpodConfig;
use crate::Result;
use crate::service::{
    BillingService, EndpointsService, PodsService, RegistryService, TemplatesService,
    VolumesService,
};

/// Inner client state that is shared via Arc for cheap cloning
#[derive(Debug)]
struct RunpodClientInner {
    config: RunpodConfig,
    client: Client,
}

/// Main Runpod API client
///
/// This client is cheap to clone as it uses Arc internally for shared state.
#[derive(Clone)]
pub struct RunpodClient {
    inner: Arc<RunpodClientInner>,
}

impl fmt::Debug for RunpodClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunpodClient")
            .field("base_url", &self.inner.config.base_url())
            .field("timeout", &self.inner.config.timeout())
            .finish()
    }
}

impl RunpodClient {
    /// Creates a new Runpod API client
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config)))]
    pub fn new(config: RunpodConfig) -> Result<Self> {
        let client = Client::builder().timeout(config.timeout()).build()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(base_url = %config.base_url(), timeout = ?config.timeout(), "Created Runpod client");

        let inner = Arc::new(RunpodClientInner { config, client });
        Ok(Self { inner })
    }

    /// Gets the pods service
    pub fn pods(&self) -> PodsService {
        PodsService::new(self.clone())
    }

    /// Gets the endpoints service
    pub fn endpoints(&self) -> EndpointsService {
        EndpointsService::new(self.clone())
    }

    /// Gets the templates service
    pub fn templates(&self) -> TemplatesService {
        TemplatesService::new(self.clone())
    }

    /// Gets the volumes service
    pub fn volumes(&self) -> VolumesService {
        VolumesService::new(self.clone())
    }

    /// Gets the container registry auth service
    pub fn container_registry_auth(&self) -> RegistryService {
        RegistryService::new(self.clone())
    }

    /// Gets the billing service
    pub fn billing(&self) -> BillingService {
        BillingService::new(self.clone())
    }

    /// Creates a GET request
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .get(&url)
            .bearer_auth(self.inner.config.api_key())
    }

    /// Creates a POST request
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .post(&url)
            .bearer_auth(self.inner.config.api_key())
    }

    /// Creates a PUT request
    #[allow(dead_code)]
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .put(&url)
            .bearer_auth(self.inner.config.api_key())
    }

    /// Creates a PATCH request
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .patch(&url)
            .bearer_auth(self.inner.config.api_key())
    }

    /// Creates a DELETE request
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .client
            .delete(&url)
            .bearer_auth(self.inner.config.api_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_client_creation() -> TestResult {
        let config = RunpodConfig::builder().with_api_key("test_key").build()?;

        let client = RunpodClient::new(config)?;
        assert!(client.inner.config.api_key() == "test_key");
        Ok(())
    }
}
