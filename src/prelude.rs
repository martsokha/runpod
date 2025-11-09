//! Prelude module for convenient imports.
//!
//! The prelude re-exports the most commonly used types and traits from the RunPod SDK,
//! allowing you to import everything you need with a single glob import.
//!
//! # Usage
//!
//! ```
//! use runpod_sdk::prelude::*;
//!
//! # fn example() -> Result<()> {
//! // Now you have access to:
//! // - RunpodClient, RunpodConfig, RunpodBuilder
//! // - All V1 service traits (PodsService, EndpointsService, etc.)
//! // - All V1 model types
//! // - Result, Error types
//! // - API version markers (V1, ApiVersion)
//!
//! let client: RunpodClient = RunpodClient::from_env()?;
//! # Ok(())
//! # }
//! ```
//!
//! # What's Included
//!
//! ## Client Types
//! - [`RunpodClient`] - Main API client
//! - [`RunpodConfig`] - Configuration builder
//! - [`RunpodBuilder`] - Builder pattern for configuration
//! - [`V1`] - Version 1 API marker (default)
//! - [`ApiVersion`] - API version trait
//!
//! ## Service Traits
//! - [`BillingService`] - Account billing and usage
//! - [`EndpointsService`] - Serverless endpoint management
//! - [`PodsService`] - Pod lifecycle operations
//! - [`RegistryService`] - Container registry authentication
//! - [`TemplatesService`] - Template management
//! - [`VolumesService`] - Network volume operations
//!
//! ## Error Types
//! - [`Result`] - Result type alias
//! - [`Error`] - Error enum
//!
//! ## Model Types
//! All model types from [`crate::model`] are re-exported.

// Client types
// Re-export all V1 model types
pub use crate::model::v1::*;
// V1 Service traits
pub use crate::service::v1::{
    BillingService, EndpointsService, PodsService, RegistryService, TemplatesService,
    VolumesService,
};
pub use crate::{ApiVersion, Error, Result, RunpodBuilder, RunpodClient, RunpodConfig, V1};
