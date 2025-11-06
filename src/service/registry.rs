use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    ContainerRegistryAuth, ContainerRegistryAuthCreateInput, ContainerRegistryAuths,
};

/// Service for managing container registry authentication.
#[derive(Debug, Clone)]
pub struct RegistryService {
    client: RunpodClient,
}

impl RegistryService {
    /// Creates a new container registry auth service.
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Creates a new container registry authentication.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::ContainerRegistryAuthCreateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = ContainerRegistryAuthCreateInput {
    ///     name: "My Docker Credentials".to_string(),
    ///     username: "my_username".to_string(),
    ///     password: "my_password".to_string(),
    /// };
    ///
    /// let auth = client.container_registry_auth().create(input).await?;
    /// println!("Created auth: {}", auth.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        input: ContainerRegistryAuthCreateInput,
    ) -> Result<ContainerRegistryAuth> {
        let response = self
            .client
            .post("/containerregistryauth")
            .json(&input)
            .send()
            .await?;
        let auth = response.json().await?;
        Ok(auth)
    }

    /// Lists all container registry authentications.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let auths = client.container_registry_auth().list().await?;
    /// println!("Found {} auths", auths.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<ContainerRegistryAuths> {
        let response = self.client.get("/containerregistryauth").send().await?;
        let auths = response.json().await?;
        Ok(auths)
    }

    /// Deletes a container registry authentication by ID.
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let auth = client.container_registry_auth().get("auth_id").await?;
    /// println!("Auth: {:?}", auth);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, container_registry_auth_id: &str) -> Result<ContainerRegistryAuth> {
        let path = format!("/containerregistryauth/{}", container_registry_auth_id);
        let response = self.client.get(&path).send().await?;
        let auth = response.json().await?;
        Ok(auth)
    }

    /// Deletes a container registry authentication
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.container_registry_auth().delete("auth_id").await?;
    /// println!("Auth deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, container_registry_auth_id: &str) -> Result<()> {
        let path = format!("/containerregistryauth/{}", container_registry_auth_id);
        self.client.delete(&path).send().await?;
        Ok(())
    }
}
