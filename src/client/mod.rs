//! RunPod API client configuration and initialization.
//!
//! This module provides the core client types for interacting with the RunPod API:
//!
//! - [`RunpodConfig`] - Configuration builder for API settings
//! - [`RunpodBuilder`] - Builder pattern for creating configurations
//! - [`RunpodClient`] - Main client for making API requests
//!
//! # Examples
//!
//! Creating a client with environment variables:
//!
//! ```no_run
//! use runpod_sdk::RunpodConfig;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunpodConfig::from_env()?.build_client()?;
//! # Ok(())
//! # }
//! ```
//!
//! Creating a client with the builder pattern:
//!
//! ```no_run
//! use runpod_sdk::RunpodConfig;
//! use std::time::Duration;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunpodConfig::builder()
//!     .with_api_key("your-api-key")
//!     .with_timeout(Duration::from_secs(30))
//!     .build_client()?;
//! # Ok(())
//! # }
//! ```

pub(crate) mod config;
mod runpod;

pub use config::{RunpodBuilder, RunpodConfig};
pub use runpod::RunpodClient;
