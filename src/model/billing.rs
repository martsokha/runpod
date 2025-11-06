use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use super::common::GpuTypeId;

/// Billing time bucket size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BucketSize {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

/// Grouping for billing records
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum BillingGrouping {
    PodId,
    EndpointId,
    GpuTypeId,
}

/// Billing record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingRecord {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_space_billed_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    pub time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_billed_ms: Option<i64>,
}

/// List of billing records
pub type BillingRecords = Vec<BillingRecord>;

/// Query parameters for pod billing
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodBillingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<GpuTypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<BillingGrouping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// Query parameters for endpoint billing
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointBillingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<Vec<GpuTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<BillingGrouping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// Query parameters for network volume billing
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeBillingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_size: Option<BucketSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
