//! Endpoint class for running serverless jobs

use std::sync::Arc;

use serde::Serialize;
use serde_json::Value;

use super::job::Job;
use super::types::EndpointHealth;
use crate::{Result, RunpodClient};

/// Class for running jobs on a specific endpoint.
///
/// # Examples
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::endpoint::Endpoint;
/// use serde_json::json;
///
/// # async fn example() -> Result<()> {
/// let client = RunpodClient::from_env()?;
/// let endpoint = Endpoint::new("YOUR_ENDPOINT_ID", &client);
///
/// let job = endpoint.run(&json!({"prompt": "Hello, world!"}));
/// let output: serde_json::Value = job.await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Endpoint {
    endpoint_id: Arc<String>,
    client: RunpodClient,
}

impl Endpoint {
    /// Creates a new Endpoint instance.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The unique identifier for the serverless endpoint
    /// * `client` - Reference to the RunpodClient
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(endpoint_id: impl Into<String>, client: &RunpodClient) -> Self {
        Self {
            endpoint_id: Arc::new(endpoint_id.into()),
            client: client.clone(),
        }
    }

    /// Returns the endpoint ID.
    pub fn endpoint_id(&self) -> &str {
        &self.endpoint_id
    }

    /// Runs a job on the endpoint.
    ///
    /// # Arguments
    ///
    /// * `input` - The input payload for the job
    ///
    /// # Returns
    ///
    /// Returns a Job instance that implements Future for retrieving results.
    /// The job submission happens when you first poll the Job (e.g., by awaiting it).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # use serde::{Deserialize, Serialize};
    /// # use serde_json::json;
    /// #
    /// # #[derive(Serialize)]
    /// # struct Input {
    /// #     prompt: String,
    /// # }
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    ///
    /// let job = endpoint.run(&Input {
    ///     prompt: "Hello, World!".to_string()
    /// });
    ///
    /// let output: serde_json::Value = job.await?;
    /// println!("Job result: {:?}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub fn run<I>(&self, input: &I) -> Result<Job>
    where
        I: Serialize,
    {
        let input_value = serde_json::to_value(input)?;

        Ok(Job::new(
            Arc::clone(&self.endpoint_id),
            input_value,
            self.client.clone(),
        ))
    }

    /// Runs a job and immediately waits for the result.
    ///
    /// This is a convenience method that runs a job and awaits its completion.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    ///
    /// let output: serde_json::Value = endpoint.run_now(&json!({"prompt": "Hello"})).await?;
    /// println!("Result: {:?}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_now<I, O>(&self, input: &I) -> Result<O>
    where
        I: Serialize,
        O: serde::de::DeserializeOwned,
    {
        let job = self.run(input)?;
        let value = job.await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Checks the health of the endpoint.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    ///
    /// let health = endpoint.health().await?;
    /// println!("Workers ready: {}", health.workers.ready);
    /// println!("Jobs in queue: {}", health.jobs.in_queue);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health(&self) -> Result<EndpointHealth> {
        let path = format!("{}/health", self.endpoint_id);

        let response = self.client.get_api(&path).send().await?;
        let response = response.error_for_status()?;
        let health: EndpointHealth = response.json().await?;

        Ok(health)
    }

    /// Purges all jobs from the endpoint's queue.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    ///
    /// let result = endpoint.purge_queue().await?;
    /// println!("Queue purged: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn purge_queue(&self) -> Result<Value> {
        let path = format!("{}/purge-queue", self.endpoint_id);

        let response = self.client.post_api(&path).send().await?;
        let response = response.error_for_status()?;
        let data: Value = response.json().await?;

        Ok(data)
    }
}

impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Endpoint")
            .field("endpoint_id", &self.endpoint_id)
            .finish()
    }
}
