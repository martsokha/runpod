//! RunPod Serverless Endpoint Runner
//!
//! This module provides functionality for running jobs on RunPod serverless endpoints.
//!
//! # Overview
//!
//! The endpoint runner allows you to:
//! - Submit jobs to serverless endpoints
//! - Track job status and retrieve results
//! - Stream real-time output from jobs
//! - Manage endpoint health and queue
//!
//! # Core Types
//!
//! - [`Endpoint`] - Main interface for running jobs on an endpoint
//! - [`Job`] - Handle for tracking and retrieving job results
//! - [`JobStatus`] - Enumeration of possible job states
//! - [`EndpointHealth`] - Health and statistics information
//!
//! # Basic Usage
//!
//! ```no_run
//! use runpod_sdk::{RunpodClient, Result};
//! use runpod_sdk::endpoint::Endpoint;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize)]
//! struct Input {
//!     prompt: String,
//! }
//!
//! #[derive(Deserialize)]
//! struct Output {
//!     text: String,
//! }
//!
//! # async fn example() -> Result<()> {
//! let client = RunpodClient::from_env()?;
//! let endpoint = Endpoint::new("YOUR_ENDPOINT_ID", &client);
//!
//! let job = endpoint.run(&Input {
//!     prompt: "Hello, world!".to_string(),
//! })?;
//!
//! let output: Output = job.output().await?;
//! println!("Result: {}", output.text);
//! # Ok(())
//! # }
//! ```
//!
//! # Checking Status
//!
//! ```no_run
//! # use runpod_sdk::{RunpodClient, Result};
//! # use runpod_sdk::endpoint::{Endpoint, JobStatus};
//! # use serde_json::json;
//! # async fn example() -> Result<()> {
//! # let client = RunpodClient::from_env()?;
//! # let endpoint = Endpoint::new("YOUR_ENDPOINT_ID", &client);
//! let job = endpoint.run(&json!({"prompt": "Hello"}))?;
//!
//! let status = job.status().await?;
//! match status {
//!     JobStatus::Completed => println!("Done"),
//!     JobStatus::InProgress => println!("Running"),
//!     JobStatus::Failed => println!("Failed"),
//!     _ => println!("Status: {}", status),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Streaming Results
//!
//! ```no_run
//! # use runpod_sdk::{RunpodClient, Result};
//! # use runpod_sdk::endpoint::{Endpoint, JobStatus};
//! # use serde_json::json;
//! # async fn example() -> Result<()> {
//! # let client = RunpodClient::from_env()?;
//! # let endpoint = Endpoint::new("YOUR_ENDPOINT_ID", &client);
//! let job = endpoint.run(&json!({"prompt": "Generate text"}))?;
//!
//! loop {
//!     let (status, chunks) = job.stream().await?;
//!     for chunk in chunks {
//!         println!("Chunk: {:?}", chunk.output);
//!     }
//!     if status.is_final() {
//!         break;
//!     }
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }
//! # Ok(())
//! # }
//! ```

mod client;
mod job;
mod types;

pub use client::Endpoint;
pub use job::Job;
pub use types::{EndpointHealth, JobOutput, JobStatus, StreamChunk};
