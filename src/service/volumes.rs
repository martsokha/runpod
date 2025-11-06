use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    NetworkVolume, NetworkVolumeCreateInput, NetworkVolumeUpdateInput, NetworkVolumes,
};

/// Service for managing network volumes
#[derive(Debug, Clone)]
pub struct VolumesService {
    client: RunpodClient,
}

impl VolumesService {
    /// Creates a new network volumes service
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Creates a new network volume
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::NetworkVolumeCreateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = NetworkVolumeCreateInput {
    ///     name: "My Volume".to_string(),
    ///     size: 50,
    ///     data_center_id: "EU-RO-1".to_string(),
    /// };
    ///
    /// let volume = client.volumes().create(input).await?;
    /// println!("Created volume: {}", volume.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: NetworkVolumeCreateInput) -> Result<NetworkVolume> {
        let response = self
            .client
            .post("/networkvolumes")
            .json(&input)
            .send()
            .await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    /// Lists network volumes
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let volumes = client.volumes().list().await?;
    /// println!("Found {} volumes", volumes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<NetworkVolumes> {
        let response = self.client.get("/networkvolumes").send().await?;
        let volumes = response.json().await?;
        Ok(volumes)
    }

    /// Gets a network volume by ID
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let volume = client.volumes().get("volume_id").await?;
    /// println!("Volume: {:?}", volume);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, network_volume_id: &str) -> Result<NetworkVolume> {
        let path = format!("/networkvolumes/{}", network_volume_id);
        let response = self.client.get(&path).send().await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    /// Updates a network volume
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::NetworkVolumeUpdateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = NetworkVolumeUpdateInput {
    ///     size: Some(100),
    ///     ..Default::default()
    /// };
    ///
    /// let volume = client.volumes().update("volume_id", input).await?;
    /// println!("Updated volume: {}", volume.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        network_volume_id: &str,
        input: NetworkVolumeUpdateInput,
    ) -> Result<NetworkVolume> {
        let path = format!("/networkvolumes/{}", network_volume_id);
        let response = self.client.patch(&path).json(&input).send().await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    /// Deletes a network volume
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.volumes().delete("volume_id").await?;
    /// println!("Volume deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, network_volume_id: &str) -> Result<()> {
        let path = format!("/networkvolumes/{}", network_volume_id);
        self.client.delete(&path).send().await?;
        Ok(())
    }
}
