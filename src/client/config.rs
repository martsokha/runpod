use derive_builder::Builder;

/// Configuration for the Runpod API client
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Config {
    /// API key for authentication
    #[builder(setter(into))]
    api_key: String,

    /// Base URL for the Runpod API
    #[builder(default = "\"https://rest.runpod.io/v1\".to_string()")]
    base_url: String,

    /// Timeout for HTTP requests in seconds
    #[builder(default = "30")]
    timeout_secs: u64,
}

impl Config {
    /// Create a new builder for Config
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Get the API key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the timeout in seconds
    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::builder().api_key("test_key").build().unwrap();

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://rest.runpod.io/v1");
        assert_eq!(config.timeout_secs(), 30);
    }

    #[test]
    fn test_config_builder_with_custom_values() {
        let config = Config::builder()
            .api_key("test_key")
            .base_url("https://custom.api.com")
            .timeout_secs(60u64)
            .build()
            .unwrap();

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://custom.api.com");
        assert_eq!(config.timeout_secs(), 60);
    }
}
