//! RunPod API data models and request/response types.
//!
//! This module contains all the data structures used to interact with the RunPod API.
//! Models are organized by API version in submodules to support future API versioning
//! while maintaining backward compatibility.
//!
//! # Features
//!
//! - **Type Safety**: All models use strongly typed fields with validation
//! - **Serialization**: Full serde support for JSON serialization/deserialization
//! - **Builder Pattern**: Many models support builder patterns for easy construction
//! - **Default Values**: Sensible defaults for optional fields

pub mod v1;
