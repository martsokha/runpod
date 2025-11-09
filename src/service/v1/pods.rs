use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::model::v1::{GetPodQuery, ListPodsQuery, Pod, PodCreateInput, PodUpdateInput, Pods};
use crate::version::V1;
use crate::{Result, RunpodClient};

/// Trait for managing pods (V1 API).
///
/// Provides methods for creating, listing, retrieving, updating, and controlling pods.
/// This trait is implemented on [`RunpodClient<V1>`](crate::RunpodClient).
pub trait PodsService {
    /// Creates a new pod.
    ///
    /// # Arguments
    ///
    /// * `input` - Configuration for the new pod
    ///
    /// # Returns
    ///
    /// Returns the created pod information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::PodCreateInput;
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = PodCreateInput {
    ///     image_name: Some("runpod/pytorch:latest".to_string()),
    ///     gpu_count: Some(1),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.create_pod(input).await?;
    /// println!("Created pod: {}", pod.id);
    /// # Ok(())
    /// # }
    /// ```
    fn create_pod(&self, input: PodCreateInput) -> impl Future<Output = Result<Pod>>;

    /// Lists pods with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a vector of pods matching the query criteria.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::ListPodsQuery;
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListPodsQuery {
    ///     include_machine: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let pods = client.list_pods(query).await?;
    /// println!("Found {} pods", pods.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_pods(&self, query: ListPodsQuery) -> impl Future<Output = Result<Pods>>;

    /// Gets a specific pod by ID.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod
    /// * `query` - Query parameters for including additional information
    ///
    /// # Returns
    ///
    /// Returns the pod information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::GetPodQuery;
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetPodQuery {
    ///     include_machine: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.get_pod("pod_id", query).await?;
    /// println!("Pod: {:?}", pod);
    /// # Ok(())
    /// # }
    /// ```
    fn get_pod(&self, pod_id: &str, query: GetPodQuery) -> impl Future<Output = Result<Pod>>;

    /// Updates an existing pod.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to update
    /// * `input` - Update parameters for the pod
    ///
    /// # Returns
    ///
    /// Returns the updated pod information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::PodUpdateInput;
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = PodUpdateInput {
    ///     name: Some("Updated Pod".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.update_pod("pod_id", input).await?;
    /// println!("Updated pod: {}", pod.id);
    /// # Ok(())
    /// # }
    /// ```
    fn update_pod(&self, pod_id: &str, input: PodUpdateInput) -> impl Future<Output = Result<Pod>>;

    /// Deletes a pod.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.delete_pod("pod_id").await?;
    /// println!("Pod deleted");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_pod(&self, pod_id: &str) -> impl Future<Output = Result<()>>;

    /// Starts or resumes a pod.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to start
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.start_pod("pod_id").await?;
    /// println!("Pod started");
    /// # Ok(())
    /// # }
    /// ```
    fn start_pod(&self, pod_id: &str) -> impl Future<Output = Result<()>>;

    /// Stops a pod.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to stop
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.stop_pod("pod_id").await?;
    /// println!("Pod stopped");
    /// # Ok(())
    /// # }
    /// ```
    fn stop_pod(&self, pod_id: &str) -> impl Future<Output = Result<()>>;

    /// Resets a pod.
    ///
    /// This operation will restart the pod with a fresh filesystem state.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to reset
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.reset_pod("pod_id").await?;
    /// println!("Pod reset");
    /// # Ok(())
    /// # }
    /// ```
    fn reset_pod(&self, pod_id: &str) -> impl Future<Output = Result<()>>;

    /// Restarts a pod.
    ///
    /// This operation will stop and then start the pod.
    ///
    /// # Arguments
    ///
    /// * `pod_id` - The unique identifier of the pod to restart
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::PodsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.restart_pod("pod_id").await?;
    /// println!("Pod restarted");
    /// # Ok(())
    /// # }
    /// ```
    fn restart_pod(&self, pod_id: &str) -> impl Future<Output = Result<()>>;
}

impl PodsService for RunpodClient<V1> {
    async fn create_pod(&self, input: PodCreateInput) -> Result<Pod> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Creating pod");

        let response = self.post("/pods").json(&input).send().await?;
        let response = response.error_for_status()?;
        let pod: Pod = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id = %pod.id, "Pod created successfully");

        Ok(pod)
    }

    async fn list_pods(&self, query: ListPodsQuery) -> Result<Pods> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing pods");

        let response = self.get("/pods").query(&query).send().await?;
        let response = response.error_for_status()?;
        let pods: Pods = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, count = pods.len(), "Pods retrieved successfully");

        Ok(pods)
    }

    async fn get_pod(&self, pod_id: &str, query: GetPodQuery) -> Result<Pod> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Getting pod");

        let path = format!("/pods/{}", pod_id);
        let response = self.get(&path).query(&query).send().await?;
        let response = response.error_for_status()?;
        let pod: Pod = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod retrieved successfully");

        Ok(pod)
    }

    async fn update_pod(&self, pod_id: &str, input: PodUpdateInput) -> Result<Pod> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Updating pod");

        let path = format!("/pods/{}", pod_id);
        let response = self.patch(&path).json(&input).send().await?;
        let response = response.error_for_status()?;
        let pod: Pod = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod updated successfully");

        Ok(pod)
    }

    async fn delete_pod(&self, pod_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Deleting pod");

        let path = format!("/pods/{}", pod_id);
        let response = self.delete(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod deleted successfully");

        Ok(())
    }

    async fn start_pod(&self, pod_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Starting pod");

        let path = format!("/pods/{}/start", pod_id);
        let response = self.post(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod started successfully");

        Ok(())
    }

    async fn stop_pod(&self, pod_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Stopping pod");

        let path = format!("/pods/{}/stop", pod_id);
        let response = self.post(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod stopped successfully");

        Ok(())
    }

    async fn reset_pod(&self, pod_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Resetting pod");

        let path = format!("/pods/{}/reset", pod_id);
        let response = self.post(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod reset successfully");

        Ok(())
    }

    async fn restart_pod(&self, pod_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, pod_id, "Restarting pod");

        let path = format!("/pods/{}/restart", pod_id);
        let response = self.post(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Pod restarted successfully");

        Ok(())
    }
}
