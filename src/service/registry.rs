use std::future::Future;

use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    ContainerRegistryAuth, ContainerRegistryAuthCreateInput, ContainerRegistryAuths,
};

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
    /// # use runpod_sdk::model::ContainerRegistryAuthCreateInput;
    /// # use runpod_sdk::service::RegistryService;
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
    /// # use runpod_sdk::service::RegistryService;
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
    /// # use runpod_sdk::service::RegistryService;
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
    /// # use runpod_sdk::service::RegistryService;
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

impl RegistryService for RunpodClient {
    async fn create_registry_auth(
        &self,
        input: ContainerRegistryAuthCreateInput,
    ) -> Result<ContainerRegistryAuth> {
        let response = self
            .post("/containerregistryauth")
            .json(&input)
            .send()
            .await?;
        let auth = response.json().await?;
        Ok(auth)
    }

    async fn list_registry_auths(&self) -> Result<ContainerRegistryAuths> {
        let response = self.get("/containerregistryauth").send().await?;
        let auths = response.json().await?;
        Ok(auths)
    }

    async fn get_registry_auth(&self, auth_id: &str) -> Result<ContainerRegistryAuth> {
        let path = format!("/containerregistryauth/{}", auth_id);
        let response = self.get(&path).send().await?;
        let auth = response.json().await?;
        Ok(auth)
    }

    async fn delete_registry_auth(&self, auth_id: &str) -> Result<()> {
        let path = format!("/containerregistryauth/{}", auth_id);
        self.delete(&path).send().await?;
        Ok(())
    }
}
