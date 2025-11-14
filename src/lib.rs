#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

// Compile-time check: ensure at least one TLS backend is enabled
#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!(
    "At least one TLS backend must be enabled. \
     Enable either the 'rustls-tls' (recommended) or 'native-tls' feature. \
     Example: cargo build --features rustls-tls"
);

mod client;
pub mod model;
#[doc(hidden)]
pub mod prelude;
#[cfg(feature = "serverless")]
#[cfg_attr(docsrs, doc(cfg(feature = "serverless")))]
pub mod serverless;
pub mod service;

pub use client::{RunpodBuilder, RunpodClient, RunpodConfig};

/// Tracing target for client-level operations (HTTP requests, client creation).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_CLIENT: &str = "runpod_sdk::client";

/// Tracing target for configuration operations (config creation, validation).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_CONFIG: &str = "runpod_sdk::config";

/// Tracing target for service-level operations (API calls, business logic).
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub const TRACING_TARGET_SERVICE: &str = "runpod_sdk::service";

#[doc(hidden)]
pub use crate::client::RunpodBuilderError;

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
/// # use runpod_sdk::{Error, Result, RunpodClient};
/// # use runpod_sdk::service::PodsService;
/// # async fn example() -> Result<()> {
/// let client: RunpodClient = RunpodClient::from_env()?;
///
/// match client.list_pods(Default::default()).await {
///     Ok(pods) => println!("Found {} pods", pods.len()),
///     Err(Error::Http(e)) => println!("Network error: {}", e),
///     Err(Error::Config(e)) => println!("Configuration error: {}", e),
///     Err(e) => println!("Other error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from the underlying HTTP client.
    ///
    /// This includes network connectivity issues, DNS resolution failures,
    /// timeout errors, and other transport-layer problems.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    ///
    /// This occurs when the SDK fails to parse API responses or serialize
    /// request payloads to/from JSON.
    #[cfg(any(feature = "graphql", feature = "serverless"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "graphql", feature = "serverless"))))]
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    ///
    /// This occurs when configuration parameters are invalid or when using
    /// the configuration builder and validation fails during the build process.
    #[error("Configuration error: {0}")]
    Config(#[from] RunpodBuilderError),

    /// Job operation error.
    ///
    /// This occurs when attempting to perform operations on a job that is in an invalid state.
    #[cfg(feature = "serverless")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serverless")))]
    #[error("Job error: {0}")]
    Job(String),
}

/// Result type for RunPod API operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>` that is used
/// throughout the RunPod SDK. All SDK methods that can fail return this Result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
