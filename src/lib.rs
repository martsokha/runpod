#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod client;
pub mod model;
mod service;

pub use client::{Config, ConfigBuilder, RunpodClient};
pub use service::{
    BillingService, RegistryService, EndpointsService, PodsService, TemplatesService,
    VolumesService,
};

use crate::client::config::ConfigBuilderError;

/// Error type for Runpod API operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Configuration builder error: {0}")]
    ConfigBuilder(String),
}

impl From<ConfigBuilderError> for Error {
    fn from(err: ConfigBuilderError) -> Self {
        Error::ConfigBuilder(err.to_string())
    }
}

/// Result type for Runpod API operations
pub type Result<T> = std::result::Result<T, Error>;
