use serde::{Deserialize, Serialize};

use super::common::*;

/// Pod resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    pub id: String,
    pub name: Option<String>,
    pub image: String,
    pub consumer_user_id: String,
    pub machine_id: String,
    pub desired_status: PodStatus,
    pub cost_per_hr: f64,
    pub adjusted_cost_per_hr: f64,
    pub gpu_count: Option<i32>,
    pub vcpu_count: f64,
    pub memory_in_gb: f64,
    pub container_disk_in_gb: i32,
    pub volume_in_gb: Option<i32>,
    pub volume_mount_path: Option<String>,
    pub volume_encrypted: bool,
    pub ports: Vec<String>,
    pub port_mappings: Option<PortMappings>,
    pub public_ip: Option<String>,
    pub env: EnvVars,
    pub docker_entrypoint: Option<Vec<String>>,
    pub docker_start_cmd: Option<Vec<String>>,
    pub interruptible: bool,
    pub locked: bool,
    pub gpu: Option<GpuInfo>,
    pub cpu_flavor_id: Option<String>,
    pub gpu_type_id: Option<String>,
    pub template_id: Option<String>,
    pub network_volume_id: Option<String>,
    pub container_registry_auth_id: Option<String>,
    pub endpoint_id: Option<String>,
    pub ai_api_id: Option<String>,
    pub sls_version: Option<i32>,
    pub last_started_at: Option<String>,
    pub last_status_change: Option<String>,
    pub machine: Option<Machine>,
    pub network_volume: Option<NetworkVolume>,
    pub savings_plans: Option<Vec<SavingsPlan>>,
}

/// List of pods
pub type Pods = Vec<Pod>;

/// Input for creating a pod
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_type: Option<CloudType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruptible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_disk_bandwidth_m_bps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_download_mbps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ram_per_gpu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_upload_mbps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_vcpu_per_gpu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_public_ip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Input for updating a pod
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Query parameters for listing pods
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPodsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_id: Option<Vec<CpuFlavorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_id: Option<Vec<DataCenterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<PodStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<Vec<GpuTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_machine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_network_volume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_savings_plans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// Query parameters for getting a single pod
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPodQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_machine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_network_volume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_savings_plans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}
