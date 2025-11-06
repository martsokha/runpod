#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod client;
pub mod model;
pub mod service;

pub use client::{RunpodBuilder, RunpodClient, RunpodConfig};

use crate::client::RunpodBuilderError;

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
/// # use runpod_sdk::{Error, Result, RunpodClient, service::PodsService};
/// # async fn example() -> Result<()> {
/// let client = RunpodClient::from_env()?;
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
    #[cfg(feature = "graphql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "graphql")))]
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    ///
    /// This occurs when configuration parameters are invalid or when using
    /// the configuration builder and validation fails during the build process.
    #[error("Configuration error: {0}")]
    Config(#[from] RunpodBuilderError),
}

/// Result type for RunPod API operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>` that is used
/// throughout the RunPod SDK. All SDK methods that can fail return this Result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
