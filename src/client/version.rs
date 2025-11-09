//! API version markers for compile-time version tracking.
//!
//! This module provides type-level API version markers that enable
//! compile-time API version enforcement through phantom types.

mod sealed {
    /// Sealed trait to prevent external implementations of `ApiVersion`.
    pub trait Sealed {}
}

/// API version marker trait.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// All service implementations are constrained to specific version markers,
/// ensuring compile-time API version safety.
///
/// # Sealed Trait
///
/// This trait uses the sealed trait pattern to prevent external implementations.
/// Only version markers defined in this crate can implement this trait.
pub trait ApiVersion: sealed::Sealed + Send + Sync + 'static {}

/// Version 1 API marker.
///
/// This is the default and current API version for the RunPod client.
/// All current service implementations are bound to this version.
///
/// # Examples
///
/// ```
/// use runpod_sdk::{RunpodClient, version::V1};
///
/// # fn example() -> runpod_sdk::Result<()> {
/// // Explicitly use V1
/// let client: RunpodClient<V1> = RunpodClient::from_env()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct V1;

impl sealed::Sealed for V1 {}
impl ApiVersion for V1 {}
