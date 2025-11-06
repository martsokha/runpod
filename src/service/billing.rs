use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    BillingRecords, EndpointBillingQuery, NetworkVolumeBillingQuery, PodBillingQuery,
};

/// Service for retrieving billing information and usage data.
///
/// The `BillingService` provides methods to query billing records for various
/// RunPod resources including Pods, Serverless endpoints, and Network volumes.
/// It allows filtering by time ranges, grouping by different criteria, and
/// retrieving detailed usage statistics for cost analysis and monitoring.
///
/// # Features
///
/// - Query Pod billing history with flexible time ranges
/// - Retrieve Serverless endpoint billing data with detailed metrics
/// - Access Network volume usage and billing information
/// - Support for different time bucket sizes (hour, day, week, month, year)
/// - Flexible grouping options for data aggregation
///
/// # Time Ranges
///
/// All billing queries support optional start and end time parameters.
/// Times should be provided in ISO 8601 format (e.g., "2024-01-01T00:00:00Z").
/// If not specified, the service will return recent billing data.
#[derive(Debug, Clone)]
pub struct BillingService {
    client: RunpodClient,
}

impl BillingService {
    /// Creates a new billing service.
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Retrieves detailed Pod billing history and usage data.
    ///
    /// This method returns billing records for GPU and CPU Pod usage, including
    /// runtime costs, resource utilization, and time-based billing information.
    /// Results can be filtered by time range and grouped by various criteria
    /// such as Pod ID, GPU type, or data center.
    ///
    /// # Parameters
    ///
    /// - `query`: Query parameters to filter and group the billing data
    ///   - `bucket_size`: Time granularity (hour, day, week, month, year)
    ///   - `grouping`: How to group the results (by Pod ID, GPU type, etc.)
    ///   - `start_time`/`end_time`: Time range filter in ISO 8601 format
    ///   - `pod_id`: Filter by specific Pod ID
    ///   - `gpu_type_id`: Filter by specific GPU type
    ///
    /// # Returns
    ///
    /// A vector of `BillingRecord` containing:
    /// - Runtime duration and costs
    /// - Resource type and specifications
    /// - Time-based billing breakdowns
    /// - Pod identification and metadata
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{PodBillingQuery, BucketSize};
    /// # async fn example() -> runpod_sdk::Result<()> {
    /// let config = RunpodConfig::from_env()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = PodBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-01-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().pods(query).await?;
    /// println!("Found {} pod billing records", records.len());
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

    /// Retrieves comprehensive Serverless endpoint billing history and metrics.
    ///
    /// This method provides detailed billing information for Serverless endpoint
    /// usage, including request processing costs, idle time charges, and scaling
    /// metrics. The data helps analyze endpoint performance and cost efficiency.
    ///
    /// # Parameters
    ///
    /// - `query`: Query parameters for filtering and grouping endpoint billing data
    ///   - `bucket_size`: Time granularity for aggregating billing data
    ///   - `grouping`: Grouping criteria (endpoint ID, template ID, etc.)
    ///   - `start_time`/`end_time`: Time range for the billing period
    ///   - `endpoint_id`: Filter by specific endpoint
    ///   - `template_id`: Filter by template used by endpoints
    ///   - `gpu_type_id`: Filter by GPU type used
    ///
    /// # Returns
    ///
    /// A vector of `BillingRecord` containing:
    /// - Request processing costs and duration
    /// - Idle time and scaling charges
    /// - Worker utilization metrics
    /// - Template and resource information
    /// - Time-based cost breakdowns
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{EndpointBillingQuery, BucketSize, BillingGrouping};
    /// # async fn example() -> runpod_sdk::Result<()> {
    /// let config = RunpodConfig::from_env()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = EndpointBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     grouping: Some(BillingGrouping::EndpointId),
    ///     start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-01-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().endpoints(query).await?;
    /// println!("Found {} endpoint billing records", records.len());
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

    /// Retrieves Network Volume billing history and storage usage metrics.
    ///
    /// This method returns billing information for Network Volume storage,
    /// including storage capacity costs, data transfer charges, and usage
    /// patterns. This data is essential for managing storage costs and
    /// optimizing volume utilization across your infrastructure.
    ///
    /// # Parameters
    ///
    /// - `query`: Query parameters for filtering volume billing data
    ///   - `bucket_size`: Time granularity for billing aggregation
    ///   - `grouping`: How to group results (by volume ID, data center, etc.)
    ///   - `start_time`/`end_time`: Time range for billing period
    ///   - `volume_id`: Filter by specific Network Volume ID
    ///   - `data_center_id`: Filter by data center location
    ///
    /// # Returns
    ///
    /// A vector of `BillingRecord` containing:
    /// - Storage capacity costs and duration
    /// - Data transfer and bandwidth charges
    /// - Volume utilization metrics
    /// - Data center and regional cost breakdown
    /// - Time-based storage billing details
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::{NetworkVolumeBillingQuery, BucketSize, BillingGrouping};
    /// # async fn example() -> runpod_sdk::Result<()> {
    /// let config = RunpodConfig::from_env()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = NetworkVolumeBillingQuery {
    ///     bucket_size: Some(BucketSize::Day),
    ///     start_time: Some("2024-01-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-01-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let records = client.billing().network_volumes(query).await?;
    /// println!("Found {} volume billing records", records.len());
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
