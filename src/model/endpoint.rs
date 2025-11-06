use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

use super::common::*;
use super::pod::Pod;
use super::template::Template;

/// Scaler type for endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum ScalerType {
    QueueDelay,
    RequestCount,
}

/// Endpoint resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub id: String,
    pub name: Option<String>,
    pub user_id: String,
    pub template_id: String,
    pub version: i32,
    pub compute_type: ComputeType,
    pub created_at: String,
    pub data_center_ids: Vec<DataCenterId>,
    pub env: Option<EnvVars>,
    pub execution_timeout_ms: i32,
    pub gpu_count: Option<i32>,
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,
    pub instance_ids: Option<Vec<String>>,
    pub idle_timeout: i32,
    pub network_volume_id: Option<String>,
    pub scaler_type: ScalerType,
    pub scaler_value: i32,
    pub workers_max: i32,
    pub workers_min: i32,
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<Vec<Pod>>,
}

/// List of endpoints.
pub type Endpoints = Vec<Endpoint>;

/// Input for creating an endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointCreateInput {
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashboot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_type: Option<ScalerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_min: Option<i32>,
}

/// Input for updating an endpoint (triggers rolling release)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashboot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_type: Option<ScalerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_min: Option<i32>,
}

/// Query parameters for listing endpoints.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEndpointsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}

/// Query parameters for getting a single endpoint.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEndpointQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}
