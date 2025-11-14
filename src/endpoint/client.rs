//! Endpoint class for running serverless jobs

use std::sync::Arc;

use serde::Serialize;

use super::job::Job;
use super::types::EndpointHealth;
use crate::{Result, RunpodClient, model};

#[cfg(feature = "tracing")]
const TRACING_TARGET: &str = "runpod_sdk::endpoint";

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
/// let job = endpoint.run(&json!({"prompt": "Hello, world!"}))?;
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
    pub fn new(endpoint_id: impl Into<String>, client: RunpodClient) -> Self {
        Self {
            endpoint_id: Arc::new(endpoint_id.into()),
            client,
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
    /// })?;
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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Checking endpoint health"
        );

        let path = format!("{}/health", self.endpoint_id);

        let response = self.client.get_api(&path).send().await?;
        let response = response.error_for_status()?;
        let health: EndpointHealth = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            workers_ready = health.workers.ready,
            jobs_in_queue = health.jobs.in_queue,
            "Endpoint health retrieved"
        );

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
    /// endpoint.purge_queue().await?;
    /// println!("Queue purged");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn purge_queue(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Purging endpoint queue"
        );

        let path = format!("{}/purge-queue", self.endpoint_id);

        let response = self.client.post_api(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Endpoint queue purged successfully"
        );

        Ok(())
    }
}

impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Endpoint")
            .field("endpoint_id", &self.endpoint_id)
            .finish()
    }
}

impl model::Endpoint {
    /// Creates an endpoint runner from this endpoint.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::service::EndpointsService;
    /// # use runpod_sdk::model::GetEndpointQuery;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = client.get_endpoint("endpoint_id", GetEndpointQuery::default()).await?;
    ///
    /// let runner = endpoint.to_runner(&client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_runner(&self, client: RunpodClient) -> Endpoint {
        Endpoint::new(&self.id, client)
    }

    /// Runs a job on this endpoint.
    ///
    /// This is a convenience method that creates a runner and submits a job in one call.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::service::EndpointsService;
    /// # use runpod_sdk::model::GetEndpointQuery;
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = client.get_endpoint("endpoint_id", GetEndpointQuery::default()).await?;
    ///
    /// let job = endpoint.run_job(&json!({"prompt": "Hello"}), &client)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn run<I>(&self, client: RunpodClient, input: &I) -> Result<Job>
    where
        I: Serialize,
    {
        let runner = self.to_runner(client);
        runner.run(input)
    }
}
