//! RunPod API service traits.
//!
//! This module contains trait definitions for all RunPod API services:
//!
//! - [`BillingService`] - Account billing and usage statistics
//! - [`EndpointsService`] - Serverless endpoint management
//! - [`PodsService`] - Pod lifecycle operations
//! - [`RegistryService`] - Container registry authentication
//! - [`TemplatesService`] - Template creation and management
//! - [`VolumesService`] - Network volume operations
//!
//! All traits are implemented on [`RunpodClient`] providing direct access to API methods.
//!
//! # Usage
//!
//! Service methods are called directly on the [`RunpodClient`]:
//!
//! ```no_run
//! use runpod_sdk::{RunpodConfig, model::ListPodsQuery};
//! use runpod_sdk::service::PodsService;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunpodConfig::from_env()?.build_client()?;
//!
//! // Call service methods directly on the client
//! let pods = client.list_pods(ListPodsQuery::default()).await?;
//! let endpoints = client.list_endpoints(Default::default()).await?;
//! let templates = client.list_templates(Default::default()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! [`RunpodClient`]: crate::RunpodClient

mod billing;
mod endpoints;
mod pods;
mod registry;
mod templates;
mod volumes;

pub use billing::BillingService;
pub use endpoints::EndpointsService;
pub use pods::PodsService;
pub use registry::RegistryService;
pub use templates::TemplatesService;
pub use volumes::VolumesService;
