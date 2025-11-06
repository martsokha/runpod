use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

/// Compute type for resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum ComputeType {
    Gpu,
    Cpu,
}

/// Cloud type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum CloudType {
    Secure,
    Community,
}

/// Status of a Pod.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum PodStatus {
    Running,
    Exited,
    Terminated,
}

/// CUDA versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CudaVersion {
    #[serde(rename = "12.8")]
    V12_8,
    #[serde(rename = "12.7")]
    V12_7,
    #[serde(rename = "12.6")]
    V12_6,
    #[serde(rename = "12.5")]
    V12_5,
    #[serde(rename = "12.4")]
    V12_4,
    #[serde(rename = "12.3")]
    V12_3,
    #[serde(rename = "12.2")]
    V12_2,
    #[serde(rename = "12.1")]
    V12_1,
    #[serde(rename = "12.0")]
    V12_0,
    #[serde(rename = "11.8")]
    V11_8,
}

/// GPU types available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuTypeId {
    #[serde(rename = "NVIDIA GeForce RTX 4090")]
    NvidiaGeForceRtx4090,
    #[serde(rename = "NVIDIA A40")]
    NvidiaA40,
    #[serde(rename = "NVIDIA RTX A5000")]
    NvidiaRtxA5000,
    #[serde(rename = "NVIDIA GeForce RTX 3090")]
    NvidiaGeForceRtx3090,
    #[serde(rename = "NVIDIA RTX A4500")]
    NvidiaRtxA4500,
    #[serde(rename = "NVIDIA RTX A6000")]
    NvidiaRtxA6000,
    #[serde(rename = "NVIDIA L40S")]
    NvidiaL40S,
    #[serde(rename = "NVIDIA L4")]
    NvidiaL4,
    #[serde(rename = "NVIDIA H100 80GB HBM3")]
    NvidiaH100_80GbHbm3,
    #[serde(rename = "NVIDIA RTX 4000 Ada Generation")]
    NvidiaRtx4000Ada,
    #[serde(rename = "NVIDIA A100 80GB PCIe")]
    NvidiaA100_80GbPcie,
    #[serde(rename = "NVIDIA A100-SXM4-80GB")]
    NvidiaA100Sxm4_80Gb,
    #[serde(rename = "NVIDIA RTX A4000")]
    NvidiaRtxA4000,
    #[serde(rename = "NVIDIA RTX 6000 Ada Generation")]
    NvidiaRtx6000Ada,
    #[serde(rename = "NVIDIA RTX 2000 Ada Generation")]
    NvidiaRtx2000Ada,
    #[serde(rename = "NVIDIA H200")]
    NvidiaH200,
    #[serde(rename = "NVIDIA L40")]
    NvidiaL40,
    #[serde(rename = "NVIDIA H100 NVL")]
    NvidiaH100Nvl,
    #[serde(rename = "NVIDIA H100 PCIe")]
    NvidiaH100Pcie,
    #[serde(rename = "NVIDIA GeForce RTX 3080 Ti")]
    NvidiaGeForceRtx3080Ti,
    #[serde(rename = "NVIDIA GeForce RTX 3080")]
    NvidiaGeForceRtx3080,
    #[serde(rename = "NVIDIA GeForce RTX 3070")]
    NvidiaGeForceRtx3070,
    #[serde(rename = "Tesla V100-PCIE-16GB")]
    TeslaV100Pcie16Gb,
    #[serde(rename = "AMD Instinct MI300X OAM")]
    AmdInstinctMi300XOam,
    #[serde(rename = "NVIDIA RTX A2000")]
    NvidiaRtxA2000,
    #[serde(rename = "Tesla V100-FHHL-16GB")]
    TeslaV100Fhhl16Gb,
    #[serde(rename = "NVIDIA GeForce RTX 4080 SUPER")]
    NvidiaGeForceRtx4080Super,
    #[serde(rename = "Tesla V100-SXM2-16GB")]
    TeslaV100Sxm2_16Gb,
    #[serde(rename = "NVIDIA GeForce RTX 4070 Ti")]
    NvidiaGeForceRtx4070Ti,
    #[serde(rename = "Tesla V100-SXM2-32GB")]
    TeslaV100Sxm2_32Gb,
    #[serde(rename = "NVIDIA RTX 4000 SFF Ada Generation")]
    NvidiaRtx4000SffAda,
    #[serde(rename = "NVIDIA RTX 5000 Ada Generation")]
    NvidiaRtx5000Ada,
    #[serde(rename = "NVIDIA GeForce RTX 5090")]
    NvidiaGeForceRtx5090,
    #[serde(rename = "NVIDIA A30")]
    NvidiaA30,
    #[serde(rename = "NVIDIA GeForce RTX 4080")]
    NvidiaGeForceRtx4080,
    #[serde(rename = "NVIDIA GeForce RTX 5080")]
    NvidiaGeForceRtx5080,
    #[serde(rename = "NVIDIA GeForce RTX 3090 Ti")]
    NvidiaGeForceRtx3090Ti,
    #[serde(rename = "NVIDIA B200")]
    NvidiaB200,
}

/// CPU flavor IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
pub enum CpuFlavorId {
    Cpu3c,
    Cpu3g,
    Cpu3m,
    Cpu5c,
    Cpu5g,
    Cpu5m,
}

/// Data center IDs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCenterId {
    #[serde(rename = "EU-RO-1")]
    EuRo1,
    #[serde(rename = "CA-MTL-1")]
    CaMtl1,
    #[serde(rename = "EU-SE-1")]
    EuSe1,
    #[serde(rename = "US-IL-1")]
    UsIl1,
    #[serde(rename = "EUR-IS-1")]
    EurIs1,
    #[serde(rename = "EU-CZ-1")]
    EuCz1,
    #[serde(rename = "US-TX-3")]
    UsTx3,
    #[serde(rename = "EUR-IS-2")]
    EurIs2,
    #[serde(rename = "US-KS-2")]
    UsKs2,
    #[serde(rename = "US-GA-2")]
    UsGa2,
    #[serde(rename = "US-WA-1")]
    UsWa1,
    #[serde(rename = "US-TX-1")]
    UsTx1,
    #[serde(rename = "CA-MTL-3")]
    CaMtl3,
    #[serde(rename = "EU-NL-1")]
    EuNl1,
    #[serde(rename = "US-TX-4")]
    UsTx4,
    #[serde(rename = "US-CA-2")]
    UsCa2,
    #[serde(rename = "US-NC-1")]
    UsNc1,
    #[serde(rename = "OC-AU-1")]
    OcAu1,
    #[serde(rename = "US-DE-1")]
    UsDe1,
    #[serde(rename = "EUR-IS-3")]
    EurIs3,
    #[serde(rename = "CA-MTL-2")]
    CaMtl2,
    #[serde(rename = "AP-JP-1")]
    ApJp1,
    #[serde(rename = "EUR-NO-1")]
    EurNo1,
    #[serde(rename = "EU-FR-1")]
    EuFr1,
    #[serde(rename = "US-KS-3")]
    UsKs3,
    #[serde(rename = "US-GA-1")]
    UsGa1,
}

/// GPU information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub id: String,
    pub count: i32,
    pub display_name: String,
    pub secure_price: f64,
    pub community_price: f64,
    pub one_month_price: f64,
    pub three_month_price: f64,
    pub six_month_price: f64,
    pub one_week_price: f64,
    pub community_spot_price: f64,
    pub secure_spot_price: f64,
}

/// CPU type information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuType {
    pub id: String,
    pub display_name: String,
    pub cores: f64,
    pub threads_per_core: f64,
    pub group_id: String,
}

/// Machine information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub min_pod_gpu_count: Option<i32>,
    pub gpu_type_id: Option<String>,
    pub gpu_type: Option<GpuInfo>,
    pub cpu_count: Option<i32>,
    pub cpu_type_id: Option<String>,
    pub cpu_type: Option<CpuType>,
    pub location: String,
    pub data_center_id: String,
    pub disk_throughput_m_bps: Option<i32>,
    pub max_download_speed_mbps: Option<i32>,
    pub max_upload_speed_mbps: Option<i32>,
    pub support_public_ip: bool,
    pub secure_cloud: bool,
    pub maintenance_start: Option<String>,
    pub maintenance_end: Option<String>,
    pub maintenance_note: Option<String>,
    pub note: Option<String>,
    pub cost_per_hr: f64,
    pub current_price_per_gpu: Option<f64>,
    pub gpu_available: Option<i32>,
    pub gpu_display_name: Option<String>,
}

/// Savings plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsPlan {
    pub cost_per_hr: f64,
    pub end_time: String,
    pub gpu_type_id: String,
    pub id: String,
    pub pod_id: String,
    pub start_time: String,
}

/// Network volume information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolume {
    pub id: String,
    pub name: String,
    pub size: i32,
    pub data_center_id: String,
}

/// Environment variables.
pub type EnvVars = HashMap<String, String>;

/// Port mappings.
pub type PortMappings = HashMap<String, i32>;
