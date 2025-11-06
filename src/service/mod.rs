//! RunPod API service implementations.
//!
//! This module contains service implementations for all RunPod API endpoints:
//!
//! - [`BillingService`] - Account billing and usage statistics
//! - [`EndpointsService`] - Serverless endpoint management
//! - [`PodsService`] - Pod lifecycle operations
//! - [`RegistryService`] - Container registry authentication
//! - [`TemplatesService`] - Template creation and management
//! - [`VolumesService`] - Network volume operations
//!
//! # Usage
//!
//! Services are accessed through the main [`RunpodClient`]:
//!
//! ```no_run
//! use runpod_sdk::{RunpodConfig, model::ListPodsQuery};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunpodConfig::from_env()?.build_client()?;
//!
//! // Access different services
//! let pods = client.pods().list(ListPodsQuery::default()).await?;
//! let endpoints = client.endpoints().list(Default::default()).await?;
//! let templates = client.templates().list(Default::default()).await?;
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
