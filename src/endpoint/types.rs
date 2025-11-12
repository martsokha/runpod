//! Types for the endpoint runner

use std::fmt;

use serde::{Deserialize, Serialize};

/// Job status indicating the current state of a serverless job.
///
/// Jobs progress through various states from submission to completion or failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    /// Job is waiting in the queue to be processed
    InQueue,
    /// Job is currently being processed by a worker
    InProgress,
    /// Job has completed successfully
    Completed,
    /// Job failed due to an error
    Failed,
    /// Job exceeded the execution timeout
    TimedOut,
    /// Job was cancelled by the user
    Cancelled,
}

impl JobStatus {
    /// Returns true if the job is in a final state (completed, failed, timed out, or cancelled)
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            JobStatus::Completed | JobStatus::Failed | JobStatus::TimedOut | JobStatus::Cancelled
        )
    }

    /// Returns true if the job completed successfully
    pub fn is_completed(&self) -> bool {
        matches!(self, JobStatus::Completed)
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::InQueue => write!(f, "IN_QUEUE"),
            JobStatus::InProgress => write!(f, "IN_PROGRESS"),
            JobStatus::Completed => write!(f, "COMPLETED"),
            JobStatus::Failed => write!(f, "FAILED"),
            JobStatus::TimedOut => write!(f, "TIMED_OUT"),
            JobStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

/// Response from the status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusResponse {
    /// Current status of the job
    pub status: JobStatus,
    /// Job output (only present when completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

/// Response from the stream endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse {
    /// Current status of the job
    pub status: JobStatus,
    /// Stream chunks
    #[serde(default)]
    pub stream: Vec<StreamChunk>,
}

/// A single chunk from a streaming response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// The output data for this chunk
    pub output: serde_json::Value,
}

/// Job output that may be returned from a completed job
pub type JobOutput = serde_json::Value;

/// Health information for an endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealth {
    /// Job statistics
    pub jobs: JobStats,
    /// Worker statistics
    pub workers: WorkerStats,
}

/// Statistics about jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStats {
    /// Number of completed jobs
    pub completed: u32,
    /// Number of failed jobs
    pub failed: u32,
    /// Number of jobs currently in progress
    pub in_progress: u32,
    /// Number of jobs waiting in queue
    pub in_queue: u32,
    /// Number of retried jobs
    pub retried: u32,
}

/// Statistics about workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    /// Number of idle workers
    pub idle: u32,
    /// Number of workers initializing
    pub initializing: u32,
    /// Number of ready workers
    pub ready: u32,
    /// Number of workers currently running jobs
    pub running: u32,
    /// Number of throttled workers
    pub throttled: u32,
}

/// Request payload for running a job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    /// The input data for the job
    pub input: serde_json::Value,
}

/// Response from submitting a job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResponse {
    /// The unique identifier for the submitted job
    pub id: String,
    /// Initial status (for runsync endpoint)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// Output (for runsync endpoint when completed immediately)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}
