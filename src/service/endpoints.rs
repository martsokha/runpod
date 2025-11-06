use std::future::Future;

use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    Endpoint, EndpointCreateInput, EndpointUpdateInput, Endpoints, GetEndpointQuery,
    ListEndpointsQuery,
};

/// Trait for managing serverless endpoints.
///
/// Provides methods for creating, listing, retrieving, updating, and deleting serverless endpoints.
/// This trait is implemented on the [`RunpodClient`](crate::client::RunpodClient).
pub trait EndpointsService {
    /// Creates a new serverless endpoint.
    ///
    /// # Arguments
    ///
    /// * `input` - Configuration for the new endpoint
    ///
    /// # Returns
    ///
    /// Returns the created endpoint information.
    ///
    /// # Example
    /// 
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::EndpointCreateInput;
    /// # use runpod_sdk::service::EndpointsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::from_env()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = EndpointCreateInput {
    ///     template_id: "template_id".to_string(),
    ///     name: Some("My Endpoint".to_string()),
    ///     workers_max: Some(3),
    ///     workers_min: Some(0),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.create_endpoint(input).await?;
    /// println!("Created endpoint: {}", endpoint.id);
    /// # Ok(())
    /// # }
    /// ```
    fn create_endpoint(&self, input: EndpointCreateInput)
    -> impl Future<Output = Result<Endpoint>>;

    /// Lists serverless endpoints with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for filtering and including additional information
    ///
    /// # Returns
    ///
    /// Returns a vector of endpoints matching the query criteria.
    ///
    /// # Example
    /// 
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::ListEndpointsQuery;
    /// # use runpod_sdk::service::EndpointsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListEndpointsQuery {
    ///     include_template: Some(true),
    ///     include_workers: Some(true),
    /// };
    ///
    /// let endpoints = client.list_endpoints(query).await?;
    /// println!("Found {} endpoints", endpoints.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_endpoints(&self, query: ListEndpointsQuery) -> impl Future<Output = Result<Endpoints>>;

    /// Gets a specific endpoint by ID.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The unique identifier of the endpoint
    /// * `query` - Query parameters for including additional information
    ///
    /// # Returns
    ///
    /// Returns the endpoint information.
    ///
    /// # Example
    /// 
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::GetEndpointQuery;
    /// # use runpod_sdk::service::EndpointsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetEndpointQuery {
    ///     include_template: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.get_endpoint("endpoint_id", query).await?;
    /// println!("Endpoint: {:?}", endpoint);
    /// # Ok(())
    /// # }
    /// ```
    fn get_endpoint(
        &self,
        endpoint_id: &str,
        query: GetEndpointQuery,
    ) -> impl Future<Output = Result<Endpoint>>;

    /// Updates an existing endpoint.
    ///
    /// This operation triggers a rolling release of the endpoint with the new configuration.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The unique identifier of the endpoint to update
    /// * `input` - Update parameters for the endpoint
    ///
    /// # Returns
    ///
    /// Returns the updated endpoint information.
    ///
    /// # Example
    /// 
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::EndpointUpdateInput;
    /// # use runpod_sdk::service::EndpointsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = EndpointUpdateInput {
    ///     workers_max: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint = client.update_endpoint("endpoint_id", input).await?;
    /// println!("Updated endpoint: {}", endpoint.id);
    /// # Ok(())
    /// # }
    /// ```
    fn update_endpoint(
        &self,
        endpoint_id: &str,
        input: EndpointUpdateInput,
    ) -> impl Future<Output = Result<Endpoint>>;

    /// Deletes an endpoint.
    ///
    /// This operation will permanently remove the endpoint and all its associated resources.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The unique identifier of the endpoint to delete
    ///
    /// # Example
    /// 
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::EndpointsService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.delete_endpoint("endpoint_id").await?;
    /// println!("Endpoint deleted");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_endpoint(&self, endpoint_id: &str) -> impl Future<Output = Result<()>>;
}

impl EndpointsService for RunpodClient {
    async fn create_endpoint(&self, input: EndpointCreateInput) -> Result<Endpoint> {
        let response = self.post("/endpoints").json(&input).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    async fn list_endpoints(&self, query: ListEndpointsQuery) -> Result<Endpoints> {
        let response = self.get("/endpoints").query(&query).send().await?;
        let endpoints = response.json().await?;
        Ok(endpoints)
    }

    async fn get_endpoint(&self, endpoint_id: &str, query: GetEndpointQuery) -> Result<Endpoint> {
        let path = format!("/endpoints/{}", endpoint_id);
        let response = self.get(&path).query(&query).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    async fn update_endpoint(
        &self,
        endpoint_id: &str,
        input: EndpointUpdateInput,
    ) -> Result<Endpoint> {
        let path = format!("/endpoints/{}", endpoint_id);
        let response = self.patch(&path).json(&input).send().await?;
        let endpoint = response.json().await?;
        Ok(endpoint)
    }

    async fn delete_endpoint(&self, endpoint_id: &str) -> Result<()> {
        let path = format!("/endpoints/{}", endpoint_id);
        self.delete(&path).send().await?;
        Ok(())
    }
}
