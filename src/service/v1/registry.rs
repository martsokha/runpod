use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::model::v1::{
    ContainerRegistryAuth, ContainerRegistryAuthCreateInput, ContainerRegistryAuths,
};
use crate::version::V1;
use crate::{Result, RunpodClient};

/// Trait for managing container registry authentication.
///
/// Provides methods for creating, listing, retrieving, and deleting container registry authentications.
/// This trait is implemented on the [`RunpodClient`](crate::client::RunpodClient).
pub trait RegistryService {
    /// Creates a new container registry authentication.
    ///
    /// # Arguments
    ///
    /// * `input` - Configuration for the new container registry authentication
    ///
    /// # Returns
    ///
    /// Returns the created container registry authentication information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::ContainerRegistryAuthCreateInput;
    /// # use runpod_sdk::service::v1::RegistryService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = ContainerRegistryAuthCreateInput {
    ///     name: "My Docker Credentials".to_string(),
    ///     username: "my_username".to_string(),
    ///     password: "my_password".to_string(),
    /// };
    ///
    /// let auth = client.create_registry_auth(input).await?;
    /// println!("Created auth: {}", auth.id);
    /// # Ok(())
    /// # }
    /// ```
    fn create_registry_auth(
        &self,
        input: ContainerRegistryAuthCreateInput,
    ) -> impl Future<Output = Result<ContainerRegistryAuth>>;

    /// Lists all container registry authentications.
    ///
    /// # Returns
    ///
    /// Returns a vector of all container registry authentications associated with the account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::RegistryService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let auths = client.list_registry_auths().await?;
    /// println!("Found {} auths", auths.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_registry_auths(&self) -> impl Future<Output = Result<ContainerRegistryAuths>>;

    /// Gets a specific container registry authentication by ID.
    ///
    /// # Arguments
    ///
    /// * `auth_id` - The unique identifier of the container registry authentication
    ///
    /// # Returns
    ///
    /// Returns the container registry authentication information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::RegistryService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let auth = client.get_registry_auth("auth_id").await?;
    /// println!("Auth: {:?}", auth);
    /// # Ok(())
    /// # }
    /// ```
    fn get_registry_auth(
        &self,
        auth_id: &str,
    ) -> impl Future<Output = Result<ContainerRegistryAuth>>;

    /// Deletes a container registry authentication.
    ///
    /// This operation will permanently remove the container registry authentication.
    ///
    /// # Arguments
    ///
    /// * `auth_id` - The unique identifier of the authentication to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::v1::RegistryService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.delete_registry_auth("auth_id").await?;
    /// println!("Auth deleted");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_registry_auth(&self, auth_id: &str) -> impl Future<Output = Result<()>>;
}

impl RegistryService for RunpodClient<V1> {
    async fn create_registry_auth(
        &self,
        input: ContainerRegistryAuthCreateInput,
    ) -> Result<ContainerRegistryAuth> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Creating registry auth");

        let response = self
            .post("/containerregistryauth")
            .json(&input)
            .send()
            .await?;
        let response = response.error_for_status()?;
        let auth: ContainerRegistryAuth = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, auth_id = %auth.id, "Registry auth created successfully");

        Ok(auth)
    }

    async fn list_registry_auths(&self) -> Result<ContainerRegistryAuths> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing registry auths");

        let response = self.get("/containerregistryauth").send().await?;
        let response = response.error_for_status()?;
        let auths: ContainerRegistryAuths = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, count = auths.len(), "Registry auths retrieved successfully");

        Ok(auths)
    }

    async fn get_registry_auth(&self, auth_id: &str) -> Result<ContainerRegistryAuth> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Getting registry auth");

        let path = format!("/containerregistryauth/{}", auth_id);
        let response = self.get(&path).send().await?;
        let response = response.error_for_status()?;
        let auth: ContainerRegistryAuth = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Registry auth retrieved successfully");

        Ok(auth)
    }

    async fn delete_registry_auth(&self, auth_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Deleting registry auth");

        let path = format!("/containerregistryauth/{}", auth_id);
        let response = self.delete(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Registry auth deleted successfully");

        Ok(())
    }
}
