//! Billing models and related types for the RunPod API.
//!
//! This module contains all the data structures and types needed to retrieve and analyze
//! billing information for RunPod resources including Pods, Serverless endpoints, and Network Volumes.
//!
//! # Overview
//!
//! RunPod's billing system provides detailed usage tracking and cost analysis across all compute resources:
//!
//! - **Time-bucketed Records**: Billing data aggregated by hour, day, week, month, or year
//! - **Multi-resource Support**: Track costs for Pods, Serverless endpoints, and Network Volumes
//! - **Flexible Grouping**: Group billing records by resource ID, GPU type, or endpoint
//! - **Advanced Filtering**: Filter by time ranges, GPU types, data centers, and resource-specific criteria
//! - **Detailed Breakdowns**: Get precise costs, disk usage, and time-based billing information
//!
//! # Core Types
//!
//! - [`BillingRecord`]: Individual billing record containing cost and usage information
//! - [`BillingRecords`]: Collection of billing records returned by API calls
//! - [`PodBillingQuery`]: Query parameters for retrieving Pod billing history
//! - [`EndpointBillingQuery`]: Query parameters for retrieving Serverless endpoint billing history
//! - [`NetworkVolumeBillingQuery`]: Query parameters for retrieving Network Volume billing history
//! - [`BucketSize`]: Time bucket granularity for aggregating billing records
//! - [`BillingGrouping`]: Grouping options for organizing billing data
//!
//! # Billing Data Structure
//!
//! All billing records share a common structure with resource-specific fields:
//!
//! - **Amount**: Cost in USD for the billing period
//! - **Time**: Start timestamp of the billing period
//! - **Usage Metrics**: Time billed, disk space usage, and resource-specific data
//! - **Resource Identifiers**: Pod ID, endpoint ID, or GPU type ID depending on grouping
//!
//! # Time Buckets
//!
//! Billing data can be aggregated into different time buckets using [`BucketSize`]:
//!
//! - **Hour**: Hourly billing records for detailed analysis
//! - **Day**: Daily aggregation (default for most queries)
//! - **Week**: Weekly summaries for trend analysis
//! - **Month**: Monthly billing for accounting purposes
//! - **Year**: Yearly totals for long-term cost tracking
//!
//! # Grouping Options
//!
//! Control how billing records are organized using [`BillingGrouping`]:
//!
//! - **By Resource ID**: Group by individual Pod, endpoint, or Network Volume
//! - **By GPU Type**: Aggregate costs across all resources using the same GPU type
//! - **By Endpoint**: Group Serverless billing by endpoint (default for Serverless)
//!
//! # Examples
//!
//! ```rust
//! use runpod_sdk::model::billing::{PodBillingQuery, BucketSize, BillingGrouping};
//!
//! // Query daily Pod billing grouped by GPU type
//! let query = PodBillingQuery {
//!     bucket_size: Some(BucketSize::Day),
//!     grouping: Some(BillingGrouping::GpuTypeId),
//!     start_time: Some("2023-01-01T00:00:00Z".to_string()),
//!     end_time: Some("2023-01-31T23:59:59Z".to_string()),
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

use super::common::GpuTypeId;

/// Time bucket size for aggregating billing records.
///
/// Determines the granularity of billing data aggregation. Each billing record
/// represents usage and costs for one time bucket period.
///
/// # Granularity Trade-offs
///
/// - **Hour**: Most detailed, suitable for real-time monitoring and debugging
/// - **Day**: Good balance of detail and overview (default for most use cases)
/// - **Week**: Weekly trends and medium-term analysis
/// - **Month**: Monthly accounting and budget tracking
/// - **Year**: Long-term cost analysis and yearly planning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
pub enum BucketSize {
    /// Hourly billing aggregation for detailed analysis.
    Hour,
    /// Daily billing aggregation (default for most queries).
    Day,
    /// Weekly billing aggregation for trend analysis.
    Week,
    /// Monthly billing aggregation for accounting purposes.
    Month,
    /// Yearly billing aggregation for long-term planning.
    Year,
}

/// Grouping strategy for organizing billing records.
///
/// Controls how individual billing records are grouped and aggregated.
/// Different grouping strategies provide different views of your usage patterns and costs.
///
/// # Grouping Strategies
///
/// - **PodId**: Individual Pod-level billing (detailed per-resource view)
/// - **EndpointId**: Individual endpoint-level billing (default for Serverless)
/// - **GpuTypeId**: Aggregate by GPU type (useful for capacity planning)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "camelCase"))]
pub enum BillingGrouping {
    /// Group billing records by individual Pod ID.
    /// Provides detailed per-Pod cost breakdown.
    PodId,
    /// Group billing records by individual endpoint ID.
    /// Default grouping for Serverless endpoint billing.
    EndpointId,
    /// Group billing records by GPU type ID.
    /// Useful for analyzing costs across different GPU types and capacity planning.
    GpuTypeId,
}

/// Individual billing record containing usage and cost information.
///
/// Represents billing data for a specific time period and resource grouping.
/// Contains both cost information and detailed usage metrics.
///
/// # Field Availability
///
/// Not all fields are populated for every record type:
/// - `disk_space_billed_gb` and `time_billed_ms` may not apply to all resource types
/// - Resource ID fields (`pod_id`, `endpoint_id`, `gpu_type_id`) depend on grouping strategy
/// - Fields are omitted (serialized as null) when not applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingRecord {
    /// The amount charged for this group during the billing period, in USD.
    /// This is the total cost after applying any applicable discounts or credits.
    pub amount: f64,

    /// The amount of disk space billed for the billing period, in gigabytes (GB).
    /// Only applicable to certain resource types. Omitted when not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_space_billed_gb: Option<i32>,

    /// The endpoint ID when grouping by endpoint ID.
    /// Only populated when using `BillingGrouping::EndpointId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,

    /// The GPU type ID when grouping by GPU type ID.
    /// Only populated when using `BillingGrouping::GpuTypeId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<String>,

    /// The Pod ID when grouping by Pod ID.
    /// Only populated when using `BillingGrouping::PodId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,

    /// The start timestamp of the period for which this billing record applies.
    /// Formatted as an ISO 8601 datetime string (e.g., "2023-01-01T00:00:00Z").
    pub time: String,

    /// The total time billed for the billing period, in milliseconds.
    /// Only applicable to time-based billing (e.g., compute resources).
    /// Omitted for storage-only resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_billed_ms: Option<i64>,
}

/// Collection of billing records returned by billing API endpoints.
///
/// This type alias represents the standard response format for all billing
/// queries, containing an array of [`BillingRecord`] instances.
pub type BillingRecords = Vec<BillingRecord>;

/// Query parameters for retrieving Pod billing history.
///
/// Use this struct to customize Pod billing queries with filtering and grouping options.
/// All fields are optional, allowing for flexible query construction.
///
/// # Default Behavior
///
/// When fields are omitted:
/// - `bucket_size`: Defaults to `BucketSize::Day`
/// - `grouping`: Defaults to `BillingGrouping::GpuTypeId`
/// - Time range: Returns recent billing data (API-defined default period)
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::billing::{PodBillingQuery, BucketSize};
///
/// // Query for a specific Pod's hourly billing
/// let query = PodBillingQuery {
///     bucket_size: Some(BucketSize::Hour),
///     pod_id: Some("xedezhzb9la3ye".to_string()),
///     start_time: Some("2023-01-01T00:00:00Z".to_string()),
///     end_time: Some("2023-01-02T00:00:00Z".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodBillingQuery {
    /// The length of each billing time bucket.
    /// Determines how billing data is aggregated over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,

    /// The end date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-31T23:59:59Z").
    /// If omitted, uses API default (typically current time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Filter to Pods with the specified GPU type attached.
    /// Only billing records for Pods using this GPU type will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<GpuTypeId>,

    /// Group the billing records by the specified field.
    /// Controls how individual billing records are organized and aggregated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<BillingGrouping>,

    /// Filter to a specific Pod by its unique identifier.
    /// When specified, only billing data for this Pod is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,

    /// The start date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-01T00:00:00Z").
    /// If omitted, uses API default (typically 30 days ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// Query parameters for retrieving Serverless endpoint billing history.
///
/// Provides comprehensive filtering options for Serverless endpoint billing data,
/// including data center filtering, image-based filtering, and template-based filtering.
///
/// # Default Behavior
///
/// When fields are omitted:
/// - `bucket_size`: Defaults to `BucketSize::Day`
/// - `grouping`: Defaults to `BillingGrouping::EndpointId`
/// - `data_center_id`: Includes all available data centers
/// - Time range: Returns recent billing data (API-defined default period)
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::billing::{EndpointBillingQuery, BucketSize, BillingGrouping};
///
/// // Query billing for endpoints in specific data centers
/// let query = EndpointBillingQuery {
///     bucket_size: Some(BucketSize::Week),
///     grouping: Some(BillingGrouping::GpuTypeId),
///     data_center_id: Some(vec!["US-TX-1".to_string(), "US-CA-2".to_string()]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointBillingQuery {
    /// The length of each billing time bucket.
    /// Determines how billing data is aggregated over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,

    /// Filter to endpoints located in any of the specified RunPod data centers.
    /// Data center IDs can be found in the Pod listing response.
    /// If omitted, includes all available data centers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_id: Option<Vec<String>>,

    /// Filter to a specific endpoint by its unique identifier.
    /// When specified, only billing data for this endpoint is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,

    /// The end date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-31T23:59:59Z").
    /// If omitted, uses API default (typically current time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Filter to endpoints with any of the specified GPU types attached.
    /// Useful for analyzing costs across different GPU configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<Vec<GpuTypeId>>,

    /// Group the billing records by the specified field.
    /// Controls how individual billing records are organized and aggregated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<BillingGrouping>,

    /// Filter to endpoints created with the specified Docker image.
    /// Useful for tracking costs of specific application deployments.
    /// Example: "runpod/pytorch:2.1.0-py3.10-cuda11.8.0-devel-ubuntu22.04"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,

    /// The start date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-01T00:00:00Z").
    /// If omitted, uses API default (typically 30 days ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Filter to endpoints created from the specified template.
    /// Useful for tracking costs of endpoints deployed from specific templates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// Query parameters for retrieving Network Volume billing history.
///
/// Network Volume billing is typically based on storage capacity and duration,
/// with simpler filtering options compared to compute resources.
///
/// # Default Behavior
///
/// When fields are omitted:
/// - `bucket_size`: Defaults to `BucketSize::Day`
/// - Time range: Returns recent billing data (API-defined default period)
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::billing::{NetworkVolumeBillingQuery, BucketSize};
///
/// // Query monthly billing for a specific Network Volume
/// let query = NetworkVolumeBillingQuery {
///     bucket_size: Some(BucketSize::Month),
///     network_volume_id: Some("agv6w2qcg7".to_string()),
///     start_time: Some("2023-01-01T00:00:00Z".to_string()),
///     end_time: Some("2023-12-31T23:59:59Z".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeBillingQuery {
    /// The length of each billing time bucket.
    /// Determines how billing data is aggregated over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,

    /// The end date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-31T23:59:59Z").
    /// If omitted, uses API default (typically current time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Filter to a specific Network Volume by its unique identifier.
    /// When specified, only billing data for this Network Volume is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,

    /// The start date of the billing period to retrieve.
    /// Must be in ISO 8601 format (e.g., "2023-01-01T00:00:00Z").
    /// If omitted, uses API default (typically 30 days ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
