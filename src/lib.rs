#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod client;
pub mod model;
pub mod service;

pub use client::{RunpodClient, RunpodConfig, RunpodConfigBuilder};

use crate::client::config::RunpodConfigBuilderError;

/// Error type for RunPod API operations.
///
/// This enum represents all possible errors that can occur when using the RunPod SDK,
/// from HTTP transport errors to API-specific failures and configuration issues.
///
/// # Examples
///
/// Handling different error types:
///
/// ```no_run
/// # use runpod_sdk::{Error, Result, RunpodClient, RunpodConfig};
/// # async fn example() -> Result<()> {
/// let config = RunpodConfig::from_env()?;
/// let client = RunpodClient::new(config)?;
///
/// match client.pods().list().await {
///     Ok(pods) => println!("Found {} pods", pods.len()),
///     Err(Error::Http(e)) => println!("Network error: {}", e),
///     Err(Error::Api(msg)) => println!("API error: {}", msg),
///     Err(e) => println!("Other error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from the underlying HTTP client.
    ///
    /// This includes network connectivity issues, DNS resolution failures,
    /// timeout errors, and other transport-layer problems.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API-level error returned by the RunPod service.
    ///
    /// This represents errors in the API response, such as authentication
    /// failures, resource not found, validation errors, etc.
    #[error("API error: {0}")]
    Api(String),

    /// JSON serialization/deserialization error.
    ///
    /// This occurs when the SDK fails to parse API responses or serialize
    /// request payloads to/from JSON.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid configuration error.
    ///
    /// This is raised when configuration parameters are invalid or missing,
    /// such as empty API keys or invalid URLs.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Configuration builder error.
    ///
    /// This occurs when using the configuration builder and validation fails
    /// during the build process.
    #[error("Configuration builder error: {0}")]
    ConfigBuilder(String),
}

impl From<RunpodConfigBuilderError> for Error {
    fn from(err: RunpodConfigBuilderError) -> Self {
        Error::ConfigBuilder(err.to_string())
    }
}

/// Result type for RunPod API operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>` that is used
/// throughout the RunPod SDK. All SDK methods that can fail return this Result type.
///
/// # Examples
///
/// Using the Result type:
///
/// ```no_run
/// # use runpod_sdk::{Result, RunpodClient, RunpodConfig};
/// # async fn example() -> Result<()> {
/// let config = RunpodConfig::from_env()?;
/// let client = RunpodClient::new(config)?;
/// let pods = client.pods().list().await?;
/// println!("Successfully retrieved {} pods", pods.len());
/// # Ok(())
/// # }
/// ```
///
/// Error propagation:
///
/// ```no_run
/// # use runpod_sdk::{Result, RunpodClient, RunpodConfig};
/// # async fn create_pod_with_error_handling() -> Result<String> {
/// let config = RunpodConfig::from_env()?; // Config errors propagate
/// let client = RunpodClient::new(config)?; // Client creation errors propagate
///
/// // API errors also propagate with the ? operator
/// let pod = client.pods().create(/* pod config */).await?;
/// Ok(pod.id)
/// # }
/// ```
pub type Result<T> = std::result::Result<T, Error>;
