//! RunPod API service traits.
//!
//! This module contains trait definitions for all RunPod API services.
//! Services are organized by API version in submodules to support future API versioning
//! while maintaining backward compatibility.
//!
//! All V1 traits are implemented on [`RunpodClient<V1>`](crate::RunpodClient) providing direct access to API methods.
//!
//! [`RunpodClient`]: crate::RunpodClient

pub mod v1;
