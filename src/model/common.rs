use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

/// Compute type for Pod resources.
///
/// Determines whether a Pod will have GPU or CPU compute resources attached.
/// When set to `GPU`, the Pod will have GPU resources and GPU-related properties
/// will be considered. When set to `CPU`, only CPU-related properties will be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum ComputeType {
    /// GPU-based compute resources.
    Gpu,
    /// CPU-based compute resources.
    Cpu,
}

/// RunPod cloud deployment type.
///
/// Determines which RunPod cloud environment the Pod will be deployed to.
/// Secure Cloud offers guaranteed availability and enterprise features,
/// while Community Cloud offers lower costs with potentially less reliability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum CloudType {
    /// Secure Cloud deployment with guaranteed resources and enterprise features.
    Secure,
    /// Community Cloud deployment with lower costs and shared resources.
    Community,
}

/// Current operational status of a Pod.
///
/// Represents the lifecycle state of a Pod, indicating whether it's actively
/// running, has exited gracefully, or has been forcibly terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "UPPERCASE"))]
pub enum PodStatus {
    /// Pod is currently running and operational.
    Running,
    /// Pod has finished execution and exited normally.
    Exited,
    /// Pod has been forcibly terminated or stopped.
    Terminated,
}

/// Available CUDA versions for GPU Pods.
///
/// Specifies which CUDA runtime version should be available on the GPU Pod.
/// This is only relevant for GPU Pods and determines software compatibility.
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

/// Available GPU hardware types for GPU Pods.
///
/// Represents the specific GPU models that can be attached to a Pod.
/// Each GPU type has different performance characteristics, memory capacity,
/// and pricing. The availability of each type varies by data center and time.
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

/// Available CPU flavor configurations for CPU Pods.
///
/// Represents different CPU configurations available for CPU-only Pods.
/// Each flavor provides different combinations of cores, memory, and performance
/// characteristics optimized for various workload types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
pub enum CpuFlavorId {
    /// 3rd generation CPU configuration - compute optimized.
    Cpu3c,
    /// 3rd generation CPU configuration - general purpose.
    Cpu3g,
    /// 3rd generation CPU configuration - memory optimized.
    Cpu3m,
    /// 5th generation CPU configuration - compute optimized.
    Cpu5c,
    /// 5th generation CPU configuration - general purpose.
    Cpu5g,
    /// 5th generation CPU configuration - memory optimized.
    Cpu5m,
}

/// RunPod data center locations.
///
/// Represents the geographic locations where RunPod has data centers.
/// The choice of data center affects latency, regulatory compliance,
/// and resource availability. Costs may also vary by location.
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

/// Detailed information about GPU resources attached to a Pod.
///
/// Contains comprehensive details about the GPU configuration including
/// hardware specifications, pricing across different billing periods,
/// and availability in different cloud types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    /// Unique identifier for this GPU type.
    pub id: String,
    /// Number of GPUs of this type attached to the Pod.
    pub count: i32,
    /// Human-readable display name for the GPU type.
    pub display_name: String,
    /// Hourly price per GPU in RunPod credits for Secure Cloud.
    pub secure_price: f64,
    /// Hourly price per GPU in RunPod credits for Community Cloud.
    pub community_price: f64,
    /// Monthly rate per GPU in RunPod credits (30-day billing).
    pub one_month_price: f64,
    /// Quarterly rate per GPU in RunPod credits (90-day billing).
    pub three_month_price: f64,
    /// Semi-annual rate per GPU in RunPod credits (180-day billing).
    pub six_month_price: f64,
    /// Weekly rate per GPU in RunPod credits (7-day billing).
    pub one_week_price: f64,
    /// Spot pricing per GPU hour in RunPod credits for Community Cloud.
    pub community_spot_price: f64,
    /// Spot pricing per GPU hour in RunPod credits for Secure Cloud.
    pub secure_spot_price: f64,
}

/// Detailed information about CPU resources for a Pod.
///
/// Contains specifications about the CPU configuration including
/// core count, threading capabilities, and organizational grouping.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuType {
    /// Unique identifier for this CPU type.
    pub id: String,
    /// Human-readable display name for the CPU type.
    pub display_name: String,
    /// Number of physical CPU cores available.
    pub cores: f64,
    /// Number of threads supported per physical core.
    pub threads_per_core: f64,
    /// Group identifier for organizing similar CPU types.
    pub group_id: String,
}

/// Detailed information about the physical machine hosting a Pod.
///
/// Contains comprehensive details about the hardware infrastructure,
/// networking capabilities, pricing, and operational status of the
/// machine where the Pod is running.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    /// Minimum number of GPUs required for Pods on this machine.
    pub min_pod_gpu_count: Option<i32>,
    /// Identifier for the GPU type available on this machine.
    pub gpu_type_id: Option<String>,
    /// Detailed information about the GPU type on this machine.
    pub gpu_type: Option<GpuInfo>,
    /// Total number of CPU cores available on this machine.
    pub cpu_count: Option<i32>,
    /// Identifier for the CPU type on this machine.
    pub cpu_type_id: Option<String>,
    /// Detailed information about the CPU type on this machine.
    pub cpu_type: Option<CpuType>,
    /// Geographic location description of this machine.
    pub location: String,
    /// Data center identifier where this machine is located.
    pub data_center_id: String,
    /// Disk I/O throughput capacity in megabytes per second.
    pub disk_throughput_m_bps: Option<i32>,
    /// Maximum network download speed in megabits per second.
    pub max_download_speed_mbps: Option<i32>,
    /// Maximum network upload speed in megabits per second.
    pub max_upload_speed_mbps: Option<i32>,
    /// Whether this machine supports public IP assignment.
    pub support_public_ip: bool,
    /// Whether this machine is in the Secure Cloud environment.
    pub secure_cloud: bool,
    /// Scheduled maintenance start time, if any.
    pub maintenance_start: Option<String>,
    /// Scheduled maintenance end time, if any.
    pub maintenance_end: Option<String>,
    /// Additional information about scheduled maintenance.
    pub maintenance_note: Option<String>,
    /// General notes or information about this machine.
    pub note: Option<String>,
    /// Current hourly cost in RunPod credits for this machine.
    pub cost_per_hr: f64,
    /// Current price per GPU hour in RunPod credits, if applicable.
    pub current_price_per_gpu: Option<f64>,
    /// Number of GPUs currently available on this machine.
    pub gpu_available: Option<i32>,
    /// Human-readable name of the GPU type on this machine.
    pub gpu_display_name: Option<String>,
}

/// A savings plan applied to reduce Pod costs.
///
/// Savings plans offer discounted pricing in exchange for longer-term
/// commitments to specific GPU types. They automatically apply to
/// eligible Pods to reduce the effective hourly cost.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsPlan {
    /// Discounted hourly cost per GPU in RunPod credits.
    pub cost_per_hr: f64,
    /// UTC timestamp when this savings plan expires.
    pub end_time: String,
    /// GPU type identifier that this savings plan applies to.
    pub gpu_type_id: String,
    /// Unique identifier for this savings plan.
    pub id: String,
    /// Pod identifier that this savings plan is currently applied to.
    pub pod_id: String,
    /// UTC timestamp when this savings plan became active.
    pub start_time: String,
}

/// A persistent network-attached storage volume.
///
/// Network volumes provide persistent storage that can be shared across
/// multiple Pods and persists beyond individual Pod lifecycles. They are
/// located in specific data centers and can be mounted to Pods in the same region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolume {
    /// Unique identifier for this network volume.
    pub id: String,
    /// User-defined name for this network volume.
    pub name: String,
    /// Storage capacity of this volume in gigabytes.
    pub size: i32,
    /// Data center where this network volume is located.
    pub data_center_id: String,
}

/// Environment variables for Pod containers.
///
/// A key-value mapping of environment variables that will be set
/// in the Pod's container runtime environment.
pub type EnvVars = HashMap<String, String>;

/// Port mappings from internal to external ports.
///
/// Maps internal container ports (as strings) to external public ports
/// (as integers) for network access to the Pod.
pub type PortMappings = HashMap<String, i32>;
