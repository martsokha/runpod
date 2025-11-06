use crate::Result;
use crate::client::RunpodClient;
use crate::model::{GetPodQuery, ListPodsQuery, Pod, PodCreateInput, PodUpdateInput, Pods};

/// Service for managing pods.
#[derive(Debug, Clone)]
pub struct PodsService {
    client: RunpodClient,
}

impl PodsService {
    /// Creates a new pods service.
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Creates a new pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::PodCreateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = PodCreateInput {
    ///     image_name: Some("runpod/pytorch:latest".to_string()),
    ///     gpu_count: Some(1),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.pods().create(input).await?;
    /// println!("Created pod: {}", pod.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: PodCreateInput) -> Result<Pod> {
        let response = self.client.post("/pods").json(&input).send().await?;
        let pod = response.json().await?;
        Ok(pod)
    }

    /// Lists pods.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::ListPodsQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListPodsQuery {
    ///     include_machine: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let pods = client.pods().list(query).await?;
    /// println!("Found {} pods", pods.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, query: ListPodsQuery) -> Result<Pods> {
        let response = self.client.get("/pods").query(&query).send().await?;
        let pods = response.json().await?;
        Ok(pods)
    }

    /// Gets a pod by ID.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::GetPodQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetPodQuery {
    ///     include_machine: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.pods().get("pod_id", query).await?;
    /// println!("Pod: {:?}", pod);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, pod_id: &str, query: GetPodQuery) -> Result<Pod> {
        let path = format!("/pods/{}", pod_id);
        let response = self.client.get(&path).query(&query).send().await?;
        let pod = response.json().await?;
        Ok(pod)
    }

    /// Updates a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::PodUpdateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = PodUpdateInput {
    ///     name: Some("Updated Pod".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let pod = client.pods().update("pod_id", input).await?;
    /// println!("Updated pod: {}", pod.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, pod_id: &str, input: PodUpdateInput) -> Result<Pod> {
        let path = format!("/pods/{}", pod_id);
        let response = self.client.patch(&path).json(&input).send().await?;
        let pod = response.json().await?;
        Ok(pod)
    }

    /// Deletes a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.pods().delete("pod_id").await?;
    /// println!("Pod deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, pod_id: &str) -> Result<()> {
        let path = format!("/pods/{}", pod_id);
        self.client.delete(&path).send().await?;
        Ok(())
    }

    /// Start or resume a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.pods().start("pod_id").await?;
    /// println!("Pod started");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self, pod_id: &str) -> Result<()> {
        let path = format!("/pods/{}/start", pod_id);
        self.client.post(&path).send().await?;
        Ok(())
    }

    /// Stops a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.pods().stop("pod_id").await?;
    /// println!("Pod stopped");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&self, pod_id: &str) -> Result<()> {
        let path = format!("/pods/{}/stop", pod_id);
        self.client.post(&path).send().await?;
        Ok(())
    }

    /// Reset a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.pods().reset("pod_id").await?;
    /// println!("Pod reset");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset(&self, pod_id: &str) -> Result<()> {
        let path = format!("/pods/{}/reset", pod_id);
        self.client.post(&path).send().await?;
        Ok(())
    }

    /// Restarts a pod.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.pods().restart("pod_id").await?;
    /// println!("Pod restarted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restart(&self, pod_id: &str) -> Result<()> {
        let path = format!("/pods/{}/restart", pod_id);
        self.client.post(&path).send().await?;
        Ok(())
    }
}
