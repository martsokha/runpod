use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    Endpoint, EndpointCreateInput, EndpointUpdateInput, Endpoints, GetEndpointQuery,
    ListEndpointsQuery,
};

/// Service for managing serverless endpoints
#[derive(Debug, Clone)]
pub struct EndpointsService {
    client: RunpodClient,
}

impl EndpointsService {
    /// Creates a new endpoints service
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Creates a new endpoint
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::EndpointCreateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = EndpointCreateInput {
    ///     template_id: "template_id".to_string(),
    ///     name: Some("My Endpoint".to_string()),
    ///     workers_max: Some(5),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.endpoints().create(input).await?;
    /// println!("Created endpoint: {}", endpoint.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: EndpointCreateInput) -> Result<Endpoint> {
        let response = self.client.post("/endpoints").json(&input).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    /// Lists endpoints
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::ListEndpointsQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListEndpointsQuery {
    ///     include_template: Some(true),
    ///     include_workers: Some(true),
    /// };
    ///
    /// let endpoints = client.endpoints().list(query).await?;
    /// println!("Found {} endpoints", endpoints.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, query: ListEndpointsQuery) -> Result<Endpoints> {
        let response = self.client.get("/endpoints").query(&query).send().await?;
        let endpoints = response.json().await?;
        Ok(endpoints)
    }

    /// Gets an endpoint by ID
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::GetEndpointQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetEndpointQuery {
    ///     include_template: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.endpoints().get("endpoint_id", query).await?;
    /// println!("Endpoint: {:?}", endpoint);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, endpoint_id: &str, query: GetEndpointQuery) -> Result<Endpoint> {
        let path = format!("/endpoints/{}", endpoint_id);
        let response = self.client.get(&path).query(&query).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    /// Updates an endpoint (triggers a rolling release)
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::EndpointUpdateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = EndpointUpdateInput {
    ///     workers_max: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.endpoints().update("endpoint_id", input).await?;
    /// println!("Updated endpoint: {}", endpoint.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, endpoint_id: &str, input: EndpointUpdateInput) -> Result<Endpoint> {
        let path = format!("/endpoints/{}", endpoint_id);
        let response = self.client.patch(&path).json(&input).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    /// Deletes an endpoint
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.endpoints().delete("endpoint_id").await?;
    /// println!("Endpoint deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, endpoint_id: &str) -> Result<()> {
        let path = format!("/endpoints/{}", endpoint_id);
        self.client.delete(&path).send().await?;
        Ok(())
    }
}
