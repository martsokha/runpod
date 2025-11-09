use std::future::Future;

use crate::model::v1::{
    BillingRecords, EndpointBillingQuery, NetworkVolumeBillingQuery, PodBillingQuery,
};
use crate::{Result, RunpodClient, V1};

/// Trait for retrieving billing information and usage data.
///
/// The `BillingService` provides methods to query billing records for various
/// RunPod resources including Pods, Serverless endpoints, and Network volumes.
/// It allows filtering by time ranges, grouping by different criteria, and
/// retrieving detailed usage statistics for cost analysis and monitoring.
///
/// This trait is implemented on the [`RunpodClient`](crate::client::RunpodClient).
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
pub trait BillingService {
    /// Retrieves detailed Pod billing history and usage data.
    ///
    /// This method returns billing records for GPU and CPU Pod usage, including
    /// runtime costs, resource utilization, and time-based billing information.
    /// Results can be filtered by time range and grouped by various criteria
    /// such as Pod ID, GPU type, or data center.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters to filter and group the billing data
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
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::{PodBillingQuery, BucketSize};
    /// # use runpod_sdk::service::v1::BillingService;
    /// # async fn example() -> Result<()> {
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
    /// let records = client.get_pod_billing(query).await?;
    /// println!("Found {} pod billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    fn get_pod_billing(
        &self,
        query: PodBillingQuery,
    ) -> impl Future<Output = Result<BillingRecords>>;

    /// Retrieves comprehensive Serverless endpoint billing history and metrics.
    ///
    /// This method provides detailed billing information for Serverless endpoint
    /// usage, including request processing costs, idle time charges, and scaling
    /// metrics. The data helps analyze endpoint performance and cost efficiency.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for filtering and grouping endpoint billing data
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
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::{EndpointBillingQuery, BucketSize, BillingGrouping};
    /// # use runpod_sdk::service::v1::BillingService;
    /// # async fn example() -> Result<()> {
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
    /// let records = client.get_endpoint_billing(query).await?;
    /// println!("Found {} endpoint billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    fn get_endpoint_billing(
        &self,
        query: EndpointBillingQuery,
    ) -> impl Future<Output = Result<BillingRecords>>;

    /// Retrieves Network Volume billing history and storage usage metrics.
    ///
    /// This method returns billing information for Network Volume storage,
    /// including storage capacity costs, data transfer charges, and usage
    /// patterns. This data is essential for managing storage costs and
    /// optimizing volume utilization across your infrastructure.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for filtering volume billing data
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
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::v1::{NetworkVolumeBillingQuery, BucketSize, BillingGrouping};
    /// # use runpod_sdk::service::v1::BillingService;
    /// # async fn example() -> Result<()> {
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
    /// let records = client.get_volume_billing(query).await?;
    /// println!("Found {} volume billing records", records.len());
    /// # Ok(())
    /// # }
    /// ```
    fn get_volume_billing(
        &self,
        query: NetworkVolumeBillingQuery,
    ) -> impl Future<Output = Result<BillingRecords>>;
}

impl BillingService for RunpodClient<V1> {
    async fn get_pod_billing(&self, query: PodBillingQuery) -> Result<BillingRecords> {
        let response = self.get("/billing/pods").query(&query).send().await?;
        let records = response.json().await?;
        Ok(records)
    }

    async fn get_endpoint_billing(&self, query: EndpointBillingQuery) -> Result<BillingRecords> {
        let response = self.get("/billing/endpoints").query(&query).send().await?;
        let records = response.json().await?;
        Ok(records)
    }

    async fn get_volume_billing(&self, query: NetworkVolumeBillingQuery) -> Result<BillingRecords> {
        let response = self
            .get("/billing/networkvolumes")
            .query(&query)
            .send()
            .await?;
        let records = response.json().await?;
        Ok(records)
    }
}
