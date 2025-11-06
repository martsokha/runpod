use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    BillingRecords, EndpointBillingQuery, NetworkVolumeBillingQuery, PodBillingQuery,
};

/// Service for retrieving billing information
#[derive(Debug, Clone)]
pub struct BillingService {
    client: RunpodClient,
}

impl BillingService {
    /// Creates a new billing service
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Retrieves Pod billing history
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{PodBillingQuery, BucketSize};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = PodBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     start_time: Some("2023-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2023-01-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().pods(query).await?;
    /// println!("Found {} billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pods(&self, query: PodBillingQuery) -> Result<BillingRecords> {
        let response = self
            .client
            .get("/billing/pods")
            .query(&query)
            .send()
            .await?;
        let records = response.json().await?;
        Ok(records)
    }

    /// Retrieve Serverless endpoint billing history
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{EndpointBillingQuery, BucketSize};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = EndpointBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     start_time: Some("2023-01-01T00:00:00Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().endpoints(query).await?;
    /// println!("Found {} billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn endpoints(&self, query: EndpointBillingQuery) -> Result<BillingRecords> {
        let response = self
            .client
            .get("/billing/endpoints")
            .query(&query)
            .send()
            .await?;
        let records = response.json().await?;
        Ok(records)
    }

    /// Retrieves Network Volume billing history
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{NetworkVolumeBillingQuery, BucketSize};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = NetworkVolumeBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     start_time: Some("2023-01-01T00:00:00Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().network_volumes(query).await?;
    /// println!("Found {} billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn network_volumes(
        &self,
        query: NetworkVolumeBillingQuery,
    ) -> Result<BillingRecords> {
        let response = self
            .client
            .get("/billing/networkvolumes")
            .query(&query)
            .send()
            .await?;
        let records = response.json().await?;
        Ok(records)
    }
}
