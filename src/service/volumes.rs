use std::future::Future;

use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    NetworkVolume, NetworkVolumeCreateInput, NetworkVolumeUpdateInput, NetworkVolumes,
};

/// Trait for managing network volumes.
///
/// Provides methods for creating, listing, retrieving, updating, and deleting network volumes.
/// This trait is implemented on the [`RunpodClient`](crate::client::RunpodClient).
pub trait VolumesService {
    /// Creates a new network volume.
    ///
    /// # Arguments
    ///
    /// * `input` - Configuration for the new network volume
    ///
    /// # Returns
    ///
    /// Returns the created network volume information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::NetworkVolumeCreateInput;
    /// # use runpod_sdk::service::VolumesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = NetworkVolumeCreateInput {
    ///     name: "My Volume".to_string(),
    ///     size: 50,
    ///     data_center_id: "EU-RO-1".to_string(),
    /// };
    ///
    /// let volume = client.create_volume(input).await?;
    /// println!("Created volume: {}", volume.id);
    /// # Ok(())
    /// # }
    /// ```
    fn create_volume(
        &self,
        input: NetworkVolumeCreateInput,
    ) -> impl Future<Output = Result<NetworkVolume>>;

    /// Lists all network volumes.
    ///
    /// # Returns
    ///
    /// Returns a vector of all network volumes associated with the account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::VolumesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let volumes = client.list_volumes().await?;
    /// println!("Found {} volumes", volumes.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_volumes(&self) -> impl Future<Output = Result<NetworkVolumes>>;

    /// Gets a specific network volume by ID.
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The unique identifier of the network volume
    ///
    /// # Returns
    ///
    /// Returns the network volume information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::VolumesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let volume = client.get_volume("volume_id").await?;
    /// println!("Volume: {:?}", volume);
    /// # Ok(())
    /// # }
    /// ```
    fn get_volume(&self, volume_id: &str) -> impl Future<Output = Result<NetworkVolume>>;

    /// Updates an existing network volume.
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The unique identifier of the volume to update
    /// * `input` - Update parameters for the network volume
    ///
    /// # Returns
    ///
    /// Returns the updated network volume information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::NetworkVolumeUpdateInput;
    /// # use runpod_sdk::service::VolumesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = NetworkVolumeUpdateInput {
    ///     name: Some("Updated Volume".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let volume = client.update_volume("volume_id", input).await?;
    /// println!("Updated volume: {}", volume.id);
    /// # Ok(())
    /// # }
    /// ```
    fn update_volume(
        &self,
        volume_id: &str,
        input: NetworkVolumeUpdateInput,
    ) -> impl Future<Output = Result<NetworkVolume>>;

    /// Deletes a network volume.
    ///
    /// This operation will permanently remove the network volume and all its data.
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The unique identifier of the volume to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::VolumesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.delete_volume("volume_id").await?;
    /// println!("Volume deleted");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_volume(&self, volume_id: &str) -> impl Future<Output = Result<()>>;
}

impl VolumesService for RunpodClient {
    async fn create_volume(&self, input: NetworkVolumeCreateInput) -> Result<NetworkVolume> {
        let response = self.post("/networkvolumes").json(&input).send().await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    async fn list_volumes(&self) -> Result<NetworkVolumes> {
        let response = self.get("/networkvolumes").send().await?;
        let volumes = response.json().await?;
        Ok(volumes)
    }

    async fn get_volume(&self, volume_id: &str) -> Result<NetworkVolume> {
        let path = format!("/networkvolumes/{}", volume_id);
        let response = self.get(&path).send().await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    async fn update_volume(
        &self,
        volume_id: &str,
        input: NetworkVolumeUpdateInput,
    ) -> Result<NetworkVolume> {
        let path = format!("/networkvolumes/{}", volume_id);
        let response = self.patch(&path).json(&input).send().await?;
        let volume = response.json().await?;
        Ok(volume)
    }

    async fn delete_volume(&self, volume_id: &str) -> Result<()> {
        let path = format!("/networkvolumes/{}", volume_id);
        self.delete(&path).send().await?;
        Ok(())
    }
}
