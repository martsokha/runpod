//! Version 1 API services.
//!
//! This module contains all service trait implementations for the V1 API.

mod billing;
mod endpoints;
mod pods;
mod registry;
mod templates;
mod volumes;

pub use billing::*;
pub use endpoints::*;
pub use pods::*;
pub use registry::*;
pub use templates::*;
pub use volumes::*;
