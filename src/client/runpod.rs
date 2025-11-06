use reqwest::{Client, RequestBuilder};
use std::time::Duration;

use super::config::Config;
use crate::Result;
use crate::service::{
    billing::BillingService, registry::RegistryService,
    endpoints::EndpointsService, pods::PodsService, templates::TemplatesService,
    volumes::VolumesService,
};

/// Main Runpod API client
#[derive(Debug, Clone)]
pub struct RunpodClient {
    config: Config,
    client: Client,
}

impl RunpodClient {
    /// Create a new Runpod API client
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config)))]
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs()))
            .build()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(base_url = %config.base_url(), timeout_secs = config.timeout_secs(), "Created Runpod client");

        Ok(Self { config, client })
    }

    /// Get the pods service
    pub fn pods(&self) -> PodsService {
        PodsService::new(self.clone())
    }

    /// Get the endpoints service
    pub fn endpoints(&self) -> EndpointsService {
        EndpointsService::new(self.clone())
    }

    /// Get the templates service
    pub fn templates(&self) -> TemplatesService {
        TemplatesService::new(self.clone())
    }

    /// Get the volumes service
    pub fn volumes(&self) -> VolumesService {
        VolumesService::new(self.clone())
    }

    /// Get the container registry auth service
    pub fn container_registry_auth(&self) -> RegistryService {
        RegistryService::new(self.clone())
    }

    /// Get the billing service
    pub fn billing(&self) -> BillingService {
        BillingService::new(self.clone())
    }

    /// Create a GET request
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.base_url(), path);
        self.client.get(&url).bearer_auth(self.config.api_key())
    }

    /// Create a POST request
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.base_url(), path);
        self.client.post(&url).bearer_auth(self.config.api_key())
    }

    /// Create a PATCH request
    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.base_url(), path);
        self.client.patch(&url).bearer_auth(self.config.api_key())
    }

    /// Create a DELETE request
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.base_url(), path);
        self.client.delete(&url).bearer_auth(self.config.api_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = Config::builder().api_key("test_key").build().unwrap();

        let client = RunpodClient::new(config);
        assert!(client.is_ok());
    }
}
