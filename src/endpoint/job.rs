//! Job tracking and result retrieval

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::types::{
    JobStatus, JobStatusResponse, RunRequest, RunResponse, StreamChunk, StreamResponse,
};
use crate::{Result, RunpodClient};

pin_project_lite::pin_project! {
    /// A job submitted to a serverless endpoint.
    ///
    /// Implements `Future` to allow awaiting the job result directly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    /// let job = endpoint.run(&json!({"prompt": "Hello"}))?;
    ///
    /// // Await the job to get the output
    /// let output: serde_json::Value = job.await?;
    /// println!("Output: {:?}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub struct Job {
        endpoint_id: Arc<String>,
        job_id: Option<String>,
        input: Option<Value>,
        client: RunpodClient,
        #[pin]
        state: JobState,
    }
}

enum JobState {
    NotSubmitted,
    Submitting,
    Polling,
    Ready(Option<Value>),
    Failed(crate::Error),
}

impl Job {
    /// Creates a new Job instance with input to be submitted
    pub(crate) fn new(endpoint_id: Arc<String>, input: Value, client: RunpodClient) -> Self {
        Self {
            endpoint_id,
            job_id: None,
            input: Some(input),
            client,
            state: JobState::NotSubmitted,
        }
    }

    /// Returns the job ID (if the job has been submitted)
    pub fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Returns the endpoint ID
    pub fn endpoint_id(&self) -> &str {
        &self.endpoint_id
    }

    /// Fetches the current job state from the specified endpoint.
    async fn fetch_job(&self, source: &str) -> Result<serde_json::Value> {
        let job_id = self.job_id.as_ref().ok_or_else(|| {
            crate::Error::Serialization(
                serde_json::from_str::<Value>("\"Job has not been submitted yet\"").unwrap_err(),
            )
        })?;
        let path = format!("{}/{}/{}", self.endpoint_id, source, job_id);

        let response = self.client.get_api(&path).send().await?;
        let response = response.error_for_status()?;
        let data: Value = response.json().await?;

        Ok(data)
    }

    /// Returns the current status of the job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::{Endpoint, JobStatus};
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    /// let job = endpoint.run(&json!({"prompt": "Hello"})).await?;
    ///
    /// let status = job.status().await?;
    /// match status {
    ///     JobStatus::Completed => println!("Job finished"),
    ///     JobStatus::Failed => println!("Job failed"),
    ///     JobStatus::InProgress => println!("Job is running"),
    ///     _ => println!("Job status: {}", status),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn status(&self) -> Result<JobStatus> {
        let data = self.fetch_job("status").await?;
        let response: JobStatusResponse = serde_json::from_value(data)?;
        Ok(response.status)
    }

    /// Returns the output of the job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::Endpoint;
    /// # use serde::{Deserialize, Serialize};
    /// # use serde_json::json;
    /// #
    /// # #[derive(Deserialize)]
    /// # struct Output {
    /// #     text: String,
    /// # }
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    /// let job = endpoint.run(&json!({"prompt": "Hello"})).await?;
    ///
    /// let output: Output = job.output().await?;
    /// println!("Result: {}", output.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn output<O>(&self) -> Result<O>
    where
        O: DeserializeOwned,
    {
        let data = self.fetch_job("status").await?;
        let response: JobStatusResponse = serde_json::from_value(data)?;

        match response.output {
            Some(output) => Ok(serde_json::from_value(output)?),
            None => Err(crate::Error::Serialization(
                serde_json::from_str::<Value>("\"Job has no output\"").unwrap_err(),
            )),
        }
    }

    /// Returns stream chunks from a streaming job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::endpoint::{Endpoint, JobStatus};
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = Endpoint::new("ENDPOINT_ID", &client);
    /// let job = endpoint.run(&json!({"prompt": "Generate text"})).await?;
    ///
    /// loop {
    ///     let (status, chunks) = job.stream().await?;
    ///
    ///     for chunk in chunks {
    ///         println!("Chunk: {:?}", chunk.output);
    ///     }
    ///
    ///     if status.is_final() {
    ///         break;
    ///     }
    ///
    ///     std::thread::sleep(std::time::Duration::from_secs(1));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stream(&self) -> Result<(JobStatus, Vec<StreamChunk>)> {
        let data = self.fetch_job("stream").await?;
        let response: StreamResponse = serde_json::from_value(data)?;
        Ok((response.status, response.stream))
    }

    /// Cancels the job.
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
    /// let job = endpoint.run(&json!({"prompt": "Long running task"})).await?;
    ///
    /// job.cancel().await?;
    /// println!("Job cancelled");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel(&self) -> Result<Value> {
        let job_id = self.job_id.as_ref().ok_or_else(|| {
            crate::Error::Serialization(
                serde_json::from_str::<Value>("\"Job has not been submitted yet\"").unwrap_err(),
            )
        })?;
        let path = format!("{}/cancel/{}", self.endpoint_id, job_id);

        let response = self.client.post_api(&path).send().await?;
        let response = response.error_for_status()?;
        let data = response.json().await?;
        Ok(data)
    }
}

impl Future for Job {
    type Output = Result<Value>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        match this.state.as_mut().get_mut() {
            JobState::NotSubmitted => {
                *this.state.get_mut() = JobState::Submitting;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            JobState::Submitting => {
                // Submit the job
                let endpoint_id = Arc::clone(this.endpoint_id);
                let input = this.input.take().expect("Input should be present");
                let client = this.client.clone();

                let fut = async move {
                    let path = format!("{}/run", endpoint_id);

                    let payload = RunRequest { input };

                    let response = client.post_api(&path).json(&payload).send().await?;

                    let response = response.error_for_status()?;
                    let run_response: RunResponse = response.json().await?;

                    Ok::<_, crate::Error>(run_response.id)
                };

                let mut pinned = Box::pin(fut);
                match pinned.as_mut().poll(cx) {
                    Poll::Ready(Ok(job_id)) => {
                        *this.job_id = Some(job_id);
                        *this.state.get_mut() = JobState::Polling;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Ready(Err(e)) => {
                        *this.state.get_mut() = JobState::Failed(e);
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            JobState::Polling => {
                // Create a future to fetch the job status
                let endpoint_id = Arc::clone(this.endpoint_id);
                let job_id = this.job_id.as_ref().expect("Job ID should be set").clone();
                let client = this.client.clone();

                let fut = async move {
                    let path = format!("{}/status/{}", endpoint_id, job_id);
                    let response = client.get_api(&path).send().await?;
                    let response = response.error_for_status()?;
                    let data: Value = response.json().await?;
                    let response: JobStatusResponse = serde_json::from_value(data)?;

                    Ok::<_, crate::Error>((response.status, response.output))
                };

                // Pin and poll the future
                let mut pinned = Box::pin(fut);
                match pinned.as_mut().poll(cx) {
                    Poll::Ready(Ok((status, output))) => {
                        if status.is_final() {
                            *this.state.get_mut() = JobState::Ready(output);
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        } else {
                            // Still polling, wake up later
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    }
                    Poll::Ready(Err(e)) => {
                        *this.state.get_mut() = JobState::Failed(e);
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            JobState::Ready(output) => {
                let output = output.take();
                match output {
                    Some(val) => Poll::Ready(Ok(val)),
                    None => Poll::Ready(Err(crate::Error::Serialization(
                        serde_json::from_str::<Value>("\"Job has no output\"").unwrap_err(),
                    ))),
                }
            }
            JobState::Failed(_) => {
                if let JobState::Failed(e) =
                    std::mem::replace(this.state.get_mut(), JobState::NotSubmitted)
                {
                    Poll::Ready(Err(e))
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("endpoint_id", &self.endpoint_id)
            .field("job_id", &self.job_id)
            .finish()
    }
}
