//! RunPod API client configuration and initialization.
//!
//! This module provides the core client types for interacting with the RunPod API:
//!
//! - [`RunpodConfig`] - Configuration builder for API settings
//! - [`RunpodBuilder`] - Builder pattern for creating configurations
//! - [`RunpodClient`] - Main client for making API requests

mod config;
mod runpod;
pub mod version;

pub use config::{RunpodBuilder, RunpodBuilderError, RunpodConfig};
pub use runpod::RunpodClient;
