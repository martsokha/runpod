//! RunPod client configuration and builder.
//!
//! This module provides the configuration types and builder pattern for creating
//! and customizing [`RunpodClient`] instances.

use std::fmt;
use std::time::Duration;

use derive_builder::Builder;

use crate::Result;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CONFIG;
use crate::client::RunpodClient;

/// Configuration for the Runpod API client.
///
/// This struct holds all the necessary configuration parameters for creating and using
/// a Runpod API client, including authentication credentials, API endpoint information,
/// and HTTP client settings.
///
/// # Examples
///
/// Creating a config with defaults:
/// ```no_run
/// # use runpod_sdk::RunpodConfig;
/// let config = RunpodConfig::builder()
///     .with_api_key("your-api-key")
///     .build()
///     .unwrap();
/// ```
///
/// Creating a config from environment:
/// ```no_run
/// # use runpod_sdk::RunpodConfig;
/// // Requires RUNPOD_API_KEY environment variable
/// let config = RunpodConfig::from_env().unwrap();
/// ```
///
/// Custom configuration:
/// ```no_run
/// # use runpod_sdk::RunpodConfig;
/// # use std::time::Duration;
/// let config = RunpodConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://custom.api.com")
///     .with_timeout(Duration::from_secs(60))
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Builder)]
#[builder(
    name = "RunpodBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate_config")
)]
pub struct RunpodConfig {
    /// API key for authentication with the Runpod API.
    ///
    /// You can obtain your API key from the Runpod dashboard.
    api_key: String,

    /// Base REST URL for the Runpod API.
    ///
    /// Defaults to the official Runpod REST API endpoint.
    #[builder(default = "Self::default_rest_url()")]
    rest_url: String,

    /// Base API URL for the Runpod serverless endpoints.
    ///
    /// Defaults to the official Runpod API endpoint.
    #[cfg(feature = "endpoint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "endpoint")))]
    #[builder(default = "Self::default_api_url()")]
    api_url: String,

    /// Base GraphQL URL for the Runpod API.
    ///
    /// Defaults to the official Runpod GraphQL API endpoint.
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    #[builder(default = "Self::default_graphql_url()")]
    graphql_url: String,

    /// Timeout for HTTP requests.
    ///
    /// Controls how long the client will wait for API responses before timing out.
    #[builder(default = "Self::default_timeout()")]
    timeout: Duration,
}

impl RunpodBuilder {
    /// Returns the default REST URL for the Runpod API.
    fn default_rest_url() -> String {
        "https://rest.runpod.io/v1".to_string()
    }

    /// Returns the default API URL for the Runpod serverless endpoints.
    #[cfg(feature = "endpoint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "endpoint")))]
    fn default_api_url() -> String {
        "https://api.runpod.io/v2".to_string()
    }

    /// Returns the default base GraphQL URL for the Runpod API.
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    fn default_graphql_url() -> String {
        "https://api.runpod.io/graphql".to_string()
    }

    /// Returns the default timeout.
    fn default_timeout() -> Duration {
        Duration::from_secs(30)
    }

    /// Validates the configuration before building.
    fn validate_config(&self) -> Result<(), String> {
        // Validate API key is not empty
        if let Some(ref api_key) = self.api_key
            && api_key.trim().is_empty()
        {
            return Err("API key cannot be empty".to_string());
        }

        // Validate timeout is reasonable
        if let Some(timeout) = self.timeout {
            if timeout.is_zero() {
                return Err("Timeout must be greater than 0".to_string());
            }
            if timeout > Duration::from_secs(300) {
                return Err("Timeout cannot exceed 300 seconds (5 minutes)".to_string());
            }
        }

        Ok(())
    }

    /// Creates a RunPod API client directly from the builder.
    ///
    /// This is a convenience method that builds the configuration and
    /// creates a client in one step. This is the recommended way to
    /// create a client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// let client = RunpodConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build_client()
    ///     .unwrap();
    /// ```
    pub fn build_client(self) -> Result<RunpodClient> {
        let config = self.build()?;
        RunpodClient::new(config)
    }
}

impl RunpodConfig {
    /// Creates a new configuration builder.
    ///
    /// This is the recommended way to construct a `RunpodConfig`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// let config = RunpodConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> RunpodBuilder {
        RunpodBuilder::default()
    }

    /// Creates a new RunPod API client using this configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// let config = RunpodConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = config.build_client().unwrap();
    /// ```
    pub fn build_client(self) -> Result<RunpodClient> {
        RunpodClient::new(self)
    }

    /// Returns the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns a masked version of the API key for safe display/logging.
    ///
    /// Shows the first 4 characters followed by "****", or just "****"
    /// if the key is shorter than 4 characters.
    pub fn masked_api_key(&self) -> String {
        if self.api_key.len() > 4 {
            format!("{}****", &self.api_key[..4])
        } else {
            "****".to_string()
        }
    }

    /// Returns the base REST URL.
    pub fn rest_url(&self) -> &str {
        &self.rest_url
    }

    /// Returns the API URL for serverless endpoints.
    #[cfg(feature = "endpoint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "endpoint")))]
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Returns the base GraphQL URL.
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    pub fn graphql_url(&self) -> &str {
        &self.graphql_url
    }

    /// Returns the timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Creates a configuration from environment variables.
    ///
    /// Reads the API key from the `RUNPOD_API_KEY` environment variable.
    /// Optionally reads `RUNPOD_REST_URL`, `RUNPOD_API_URL` (with endpoint feature),
    /// `RUNPOD_GRAPHQL_URL` (with graphql feature), and `RUNPOD_TIMEOUT_SECS` if set.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `RUNPOD_API_KEY` environment variable is not set
    /// - Any environment variable contains an invalid value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::RunpodConfig;
    /// // Set environment variable first:
    /// // export RUNPOD_API_KEY=your-api-key
    /// let config = RunpodConfig::from_env().unwrap();
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn from_env() -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CONFIG, "Loading configuration from environment");

        let api_key = std::env::var("RUNPOD_API_KEY").map_err(|_| {
            #[cfg(feature = "tracing")]
            tracing::error!(target: TRACING_TARGET_CONFIG, "RUNPOD_API_KEY environment variable not set");

            RunpodBuilderError::ValidationError(
                "RUNPOD_API_KEY environment variable not set".to_string(),
            )
        })?;

        let mut builder = Self::builder().with_api_key(api_key);

        // Optional: custom REST URL (also support legacy RUNPOD_BASE_URL)
        if let Ok(rest_url) = std::env::var("RUNPOD_REST_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: TRACING_TARGET_CONFIG, rest_url = %rest_url, "Using custom REST URL");

            builder = builder.with_rest_url(rest_url);
        } else if let Ok(base_url) = std::env::var("RUNPOD_BASE_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: TRACING_TARGET_CONFIG, base_url = %base_url, "Using custom base URL (legacy)");

            builder = builder.with_rest_url(base_url);
        }

        // Optional: custom API URL for serverless endpoints
        #[cfg(feature = "endpoint")]
        if let Ok(api_url) = std::env::var("RUNPOD_API_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                target: TRACING_TARGET_CONFIG,
                api_url = %api_url,
                "Using custom API URL"
            );

            builder = builder.with_api_url(api_url);
        }

        // Optional: custom GraphQL URL
        #[cfg(feature = "graphql")]
        if let Ok(graphql_url) = std::env::var("RUNPOD_GRAPHQL_URL") {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                target: TRACING_TARGET_CONFIG,
                graphql_url = %graphql_url,
                "Using custom GraphQL URL"
            );

            builder = builder.with_graphql_url(graphql_url);
        }

        // Optional: custom timeout
        if let Ok(timeout_str) = std::env::var("RUNPOD_TIMEOUT_SECS") {
            let timeout_secs = timeout_str.parse::<u64>().map_err(|_| {
                #[cfg(feature = "tracing")]
                tracing::error!(target: TRACING_TARGET_CONFIG, timeout_str = %timeout_str, "Invalid RUNPOD_TIMEOUT_SECS value");

                RunpodBuilderError::ValidationError(format!(
                    "Invalid RUNPOD_TIMEOUT_SECS value: {}",
                    timeout_str
                ))
            })?;

            #[cfg(feature = "tracing")]
            tracing::debug!(target: TRACING_TARGET_CONFIG, timeout_secs, "Using custom timeout");

            builder = builder.with_timeout(Duration::from_secs(timeout_secs));
        }

        let config = builder.build()?;

        #[cfg(feature = "tracing")]
        tracing::info!(target: TRACING_TARGET_CONFIG,
            rest_url = %config.rest_url(),
            timeout = ?config.timeout(),
            "Configuration loaded successfully from environment"
        );

        Ok(config)
    }
}

impl fmt::Debug for RunpodConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("RunpodConfig");
        debug_struct
            .field("api_key", &self.masked_api_key())
            .field("rest_url", &self.rest_url)
            .field("timeout", &self.timeout);

        #[cfg(feature = "endpoint")]
        debug_struct.field("api_url", &self.api_url);

        #[cfg(feature = "graphql")]
        debug_struct.field("graphql_url", &self.graphql_url);

        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() -> Result<()> {
        let config = RunpodConfig::builder().with_api_key("test_key").build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.rest_url(), "https://rest.runpod.io/v1");
        #[cfg(feature = "endpoint")]
        assert_eq!(config.api_url(), "https://api.runpod.io/v2");
        #[cfg(feature = "graphql")]
        assert_eq!(config.graphql_url(), "https://api.runpod.io/graphql");
        assert_eq!(config.timeout(), Duration::from_secs(30));

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = RunpodConfig::builder()
            .with_api_key("test_key")
            .with_rest_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.rest_url(), "https://custom.api.com");
        assert_eq!(config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let result = RunpodConfig::builder().with_api_key("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let result = RunpodConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(0))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_excessive_timeout() {
        let result = RunpodConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(400))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_builder_with_all_options() -> Result<()> {
        let config = RunpodConfig::builder()
            .with_api_key("test_key_comprehensive")
            .with_rest_url("https://api.custom-domain.com/v2")
            .with_timeout(Duration::from_secs(120))
            .build()?;

        assert_eq!(config.api_key(), "test_key_comprehensive");
        assert_eq!(config.rest_url(), "https://api.custom-domain.com/v2");
        assert_eq!(config.timeout(), Duration::from_secs(120));

        Ok(())
    }

    #[test]
    fn test_config_builder_defaults() -> Result<()> {
        let config = RunpodConfig::builder().with_api_key("test_key").build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.rest_url(), "https://rest.runpod.io/v1");
        assert_eq!(config.timeout(), Duration::from_secs(30));

        Ok(())
    }
}
