//! Pod models and related types for the RunPod API.
//!
//! This module contains all the data structures and types needed to work with RunPod Pods,
//! which are containerized compute instances that can be deployed with either GPU or CPU resources.
//!
//! # Overview
//!
//! Pods are the fundamental compute units in RunPod's infrastructure. They provide:
//!
//! - **Flexible Compute**: Choose between GPU-accelerated or CPU-only instances
//! - **Multiple Cloud Types**: Deploy to Secure Cloud (guaranteed resources) or Community Cloud (cost-effective)
//! - **Persistent Storage**: Container disk (ephemeral) and Pod volumes (persistent across restarts)
//! - **Network Volumes**: Shared, persistent storage that can be attached to multiple Pods
//! - **Custom Environments**: Configure Docker images, environment variables, and startup commands
//! - **Cost Optimization**: Interruptible (spot) instances and Savings Plans for reduced costs
//!
//! # Core Types
//!
//! - [`Pod`]: The main Pod resource containing all configuration and status information
//! - [`PodCreateInput`]: Parameters for creating new Pods with comprehensive configuration options
//! - [`PodUpdateInput`]: Parameters for updating existing Pods (triggers a reset)
//! - [`ListPodsQuery`]: Query parameters for filtering and customizing Pod list operations
//! - [`GetPodQuery`]: Query parameters for retrieving individual Pods with additional details
//!
//! # Pod Lifecycle
//!
//! 1. **Creation**: Use [`PodCreateInput`] to specify compute requirements, image, and configuration
//! 2. **Running**: Monitor status via [`PodStatus`] (Running, Exited, Terminated)
//! 3. **Management**: Update configuration, start/stop, reset, or restart operations
//! 4. **Termination**: Delete when no longer needed to stop billing
//!
//! # Compute Types
//!
//! - **GPU Pods**: Accelerated compute with NVIDIA GPUs for AI/ML workloads
//!   - Choose from various GPU types ([`GpuTypeId`]) including RTX, Tesla, and H100 series
//!   - Specify GPU count, memory per GPU, and CUDA version requirements
//!   - Access to high-performance networking and specialized AI frameworks
//!
//! - **CPU Pods**: Cost-effective compute for general-purpose workloads
//!   - Various CPU flavors ([`CpuFlavorId`]) optimized for compute, memory, or general use
//!   - Flexible vCPU allocation and memory configurations
//!   - Ideal for web services, batch processing, and development environments
//!
//! # Storage Options
//!
//! - **Container Disk**: Fast local storage that's wiped on Pod restart
//! - **Pod Volume**: Persistent local storage that survives restarts
//! - **Network Volume**: Shared persistent storage accessible across multiple Pods
//!
//! # Examples
//!
//! ## Creating a GPU Pod for AI/ML
//!
//! ```rust
//! use runpod_sdk::model::pod::PodCreateInput;
//! use runpod_sdk::model::common::{ComputeType, CloudType, GpuTypeId};
//!
//! let gpu_pod = PodCreateInput {
//!     name: Some("ml-training-pod".to_string()),
//!     image_name: Some("runpod/pytorch:2.1.0-py3.10-cuda11.8.0-devel".to_string()),
//!     compute_type: Some(ComputeType::Gpu),
//!     cloud_type: Some(CloudType::Secure),
//!     gpu_count: Some(1),
//!     gpu_type_ids: Some(vec![GpuTypeId::NvidiaGeForceRtx4090]),
//!     container_disk_in_gb: Some(100),
//!     volume_in_gb: Some(50),
//!     ports: Some(vec!["8888/http".to_string(), "22/tcp".to_string()]),
//!     env: Some([("JUPYTER_ENABLE_LAB".to_string(), "yes".to_string())].into()),
//!     ..Default::default()
//! };
//! ```
//!
//! ## Creating a CPU Pod for web services
//!
//! ```rust
//! use runpod_sdk::model::pod::PodCreateInput;
//! use runpod_sdk::model::common::{ComputeType, CloudType, CpuFlavorId};
//!
//! let web_pod = PodCreateInput {
//!     name: Some("web-server".to_string()),
//!     image_name: Some("nginx:alpine".to_string()),
//!     compute_type: Some(ComputeType::Cpu),
//!     cloud_type: Some(CloudType::Community),
//!     cpu_flavor_ids: Some(vec![CpuFlavorId::Cpu3g]),
//!     vcpu_count: Some(2),
//!     container_disk_in_gb: Some(20),
//!     ports: Some(vec!["80/http".to_string()]),
//!     ..Default::default()
//! };
//! ```
//!
//! ## Querying Pods with filters
//!
//! ```rust
//! use runpod_sdk::model::pod::ListPodsQuery;
//! use runpod_sdk::model::common::{ComputeType, PodStatus};
//!
//! let query = ListPodsQuery {
//!     compute_type: Some(ComputeType::Gpu),
//!     desired_status: Some(PodStatus::Running),
//!     include_machine: Some(true),
//!     include_savings_plans: Some(true),
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};

use super::common::*;

/// A Pod resource representing a containerized compute instance on RunPod.
///
/// Pods are the fundamental compute units in RunPod, providing either GPU or CPU-based
/// computing resources. They can be configured with various specifications including
/// compute type, memory, storage, networking, and environment settings.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::pod::Pod;
///
/// // Pod instances are typically obtained from API responses
/// // when listing, creating, or retrieving pods
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    /// A unique string identifying the Pod.
    pub id: String,
    /// A user-defined name for the Pod. The name does not need to be unique.
    pub name: Option<String>,
    /// The image tag for the container run on the Pod.
    pub image: String,
    /// A unique string identifying the RunPod user who rents the Pod.
    pub consumer_user_id: String,
    /// A unique string identifying the host machine the Pod is running on.
    pub machine_id: String,
    /// The current expected status of the Pod.
    pub desired_status: PodStatus,
    /// The cost in RunPod credits per hour of running the Pod.
    /// Note that the actual cost may be lower if Savings Plans are applied.
    pub cost_per_hr: f64,
    /// The effective cost in RunPod credits per hour of running the Pod,
    /// adjusted by active Savings Plans.
    pub adjusted_cost_per_hr: f64,
    /// The number of GPUs attached to the Pod (if it's a GPU Pod).
    pub gpu_count: Option<i32>,
    /// The number of virtual CPUs attached to the Pod.
    pub vcpu_count: f64,
    /// The amount of RAM, in gigabytes (GB), attached to the Pod.
    pub memory_in_gb: f64,
    /// The amount of disk space, in gigabytes (GB), allocated on the container disk.
    /// The data on the container disk is wiped when the Pod restarts.
    pub container_disk_in_gb: i32,
    /// The amount of disk space, in gigabytes (GB), allocated on the Pod volume.
    /// The data on the Pod volume is persisted across Pod restarts.
    pub volume_in_gb: Option<i32>,
    /// The absolute path where the network volume is mounted in the filesystem.
    pub volume_mount_path: Option<String>,
    /// Whether the local network volume of the Pod is encrypted.
    /// Can only be set when creating a Pod.
    pub volume_encrypted: bool,
    /// A list of ports exposed on the Pod. Each port is formatted as
    /// `[port number]/[protocol]`. Protocol can be either `http` or `tcp`.
    pub ports: Vec<String>,
    /// A mapping of internal ports to public ports on the Pod.
    /// For example, `{"22": 10341}` means that port 22 on the Pod is mapped
    /// to port 10341 and is publicly accessible at `[public ip]:10341`.
    pub port_mappings: Option<PortMappings>,
    /// The public IP address of the Pod. If the Pod is still initializing,
    /// this IP is not yet determined and will be empty.
    pub public_ip: Option<String>,
    /// Environment variables for the Pod container.
    pub env: EnvVars,
    /// If specified, overrides the ENTRYPOINT for the Docker image run on the Pod.
    /// If empty, uses the ENTRYPOINT defined in the image.
    pub docker_entrypoint: Option<Vec<String>>,
    /// If specified, overrides the start CMD for the Docker image run on the Pod.
    /// If empty, uses the start CMD defined in the image.
    pub docker_start_cmd: Option<Vec<String>>,
    /// Describes how the Pod is rented. An interruptible Pod can be rented at
    /// a lower cost but can be stopped at any time to free up resources for
    /// another Pod. A reserved Pod is rented at a higher cost but runs until
    /// it exits or is manually stopped.
    pub interruptible: bool,
    /// Whether the Pod is locked. Locking a Pod disables stopping or resetting it.
    pub locked: bool,
    /// GPU information if the Pod has GPUs attached.
    pub gpu: Option<GpuInfo>,
    /// If the Pod is a CPU Pod, the unique string identifying the CPU flavor
    /// the Pod is running on.
    pub cpu_flavor_id: Option<String>,
    /// The GPU type ID if the Pod has GPUs attached.
    pub gpu_type_id: Option<String>,
    /// If the Pod is created with a template, the unique string identifying that template.
    pub template_id: Option<String>,
    /// The unique string identifying the network volume attached to the Pod, if any.
    pub network_volume_id: Option<String>,
    /// If the Pod is created with a container registry auth, the unique string
    /// identifying that container registry auth.
    pub container_registry_auth_id: Option<String>,
    /// If the Pod is a Serverless worker, a unique string identifying the
    /// associated endpoint.
    pub endpoint_id: Option<String>,
    /// Synonym for `endpoint_id` (legacy name).
    pub ai_api_id: Option<String>,
    /// If the Pod is a Serverless worker, the version of the associated endpoint.
    pub sls_version: Option<i32>,
    /// The UTC timestamp when the Pod was last started.
    pub last_started_at: Option<String>,
    /// A string describing the last lifecycle event on the Pod.
    pub last_status_change: Option<String>,
    /// Information about the machine the Pod is running on.
    pub machine: Option<Machine>,
    /// If a network volume is attached to the Pod, information about the network volume.
    pub network_volume: Option<NetworkVolume>,
    /// The list of active Savings Plans applied to the Pod. If none are applied, the list is empty.
    pub savings_plans: Option<Vec<SavingsPlan>>,
}

/// List of pods.
pub type Pods = Vec<Pod>;

/// Input parameters for creating a new Pod.
///
/// This struct contains all the configuration options available when creating a Pod,
/// including compute specifications, networking, storage, and deployment preferences.
/// Most fields are optional and will use RunPod defaults if not specified.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::pod::PodCreateInput;
/// use runpod_sdk::model::common::{ComputeType, CloudType};
///
/// let create_input = PodCreateInput {
///     name: Some("my-pod".to_string()),
///     image_name: Some("runpod/pytorch:latest".to_string()),
///     compute_type: Some(ComputeType::Gpu),
///     cloud_type: Some(CloudType::Secure),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateInput {
    /// If the created Pod is a GPU Pod, a list of acceptable CUDA versions.
    /// If not set, any CUDA version is acceptable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,
    /// Set to `SECURE` to create the Pod in Secure Cloud. Set to `COMMUNITY`
    /// to create the Pod in Community Cloud.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_type: Option<CloudType>,
    /// Set to `GPU` to create a GPU Pod. Set to `CPU` to create a CPU Pod.
    /// If set to `CPU`, the Pod will not have a GPU attached and GPU-related
    /// properties will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    /// The amount of disk space, in gigabytes (GB), to allocate on the container disk.
    /// The data on the container disk is wiped when the Pod restarts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,
    /// Registry credentials ID for private container registries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,
    /// A list of country codes where the created Pod can be located.
    /// If not set, the Pod can be located in any country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// If the created Pod is a CPU Pod, a list of RunPod CPU flavors which
    /// can be attached to the Pod. The order determines the rental priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,
    /// If the created Pod is a CPU Pod, set to `availability` to respond to
    /// current CPU flavor availability. Set to `custom` to always try to rent
    /// CPU flavors in the order specified in `cpu_flavor_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_priority: Option<String>,
    /// A list of RunPod data center IDs where the created Pod can be located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,
    /// Set to `availability` to respond to current machine availability.
    /// Set to `custom` to always try to rent machines from data centers
    /// in the order specified in `data_center_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_priority: Option<String>,
    /// If specified, overrides the ENTRYPOINT for the Docker image.
    /// If empty, uses the ENTRYPOINT defined in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,
    /// If specified, overrides the start CMD for the Docker image.
    /// If empty, uses the start CMD defined in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,
    /// Environment variables for the Pod container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,
    /// Set to true to enable global networking for the created Pod.
    /// Currently only available for On-Demand GPU Pods on some Secure Cloud data centers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networking: Option<bool>,
    /// If the created Pod is a GPU Pod, the number of GPUs attached to the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,
    /// If the created Pod is a GPU Pod, a list of RunPod GPU types which
    /// can be attached to the Pod. The order determines the rental priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,
    /// If the created Pod is a GPU Pod, set to `availability` to respond to
    /// current GPU type availability. Set to `custom` to always try to rent
    /// GPU types in the order specified in `gpu_type_ids`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_priority: Option<String>,
    /// The image tag for the container run on the created Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// Set to true to create an interruptible or spot Pod. An interruptible Pod
    /// can be rented at a lower cost but can be stopped at any time to free up
    /// resources for another Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruptible: Option<bool>,
    /// Set to true to lock the Pod. Locking a Pod disables stopping or resetting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The minimum disk bandwidth, in megabytes per second (MBps), for the created Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_disk_bandwidth_m_bps: Option<f64>,
    /// The minimum download speed, in megabits per second (Mbps), for the created Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_download_mbps: Option<f64>,
    /// If the created Pod is a GPU Pod, the minimum amount of RAM, in gigabytes (GB),
    /// allocated to the Pod for each GPU attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ram_per_gpu: Option<i32>,
    /// The minimum upload speed, in megabits per second (Mbps), for the created Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_upload_mbps: Option<f64>,
    /// If the created Pod is a GPU Pod, the minimum number of virtual CPUs
    /// allocated to the Pod for each GPU attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_vcpu_per_gpu: Option<i32>,
    /// A user-defined name for the created Pod. The name does not need to be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique string identifying the network volume to attach to the created Pod.
    /// If attached, a network volume replaces the Pod network volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    /// A list of ports exposed on the created Pod. Each port is formatted as
    /// `[port number]/[protocol]`. Protocol can be either `http` or `tcp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    /// If the created Pod is on Community Cloud, set to true if you need the Pod
    /// to expose a public IP address. On Secure Cloud, the Pod will always have
    /// a public IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_public_ip: Option<bool>,
    /// If the Pod is created with a template, the unique string identifying that template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// If the created Pod is a CPU Pod, the number of vCPUs allocated to the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    /// The amount of disk space, in gigabytes (GB), to allocate on the Pod volume.
    /// The data on the Pod volume is persisted across Pod restarts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    /// The absolute path where the network volume will be mounted in the filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Input parameters for updating an existing Pod.
///
/// This struct contains the configuration options that can be modified for
/// an existing Pod. Note that updating a Pod will trigger a reset.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::pod::PodUpdateInput;
///
/// let update_input = PodUpdateInput {
///     name: Some("updated-pod-name".to_string()),
///     locked: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodUpdateInput {
    /// The amount of disk space, in gigabytes (GB), to allocate on the container disk.
    /// The data on the container disk is wiped when the Pod restarts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,
    /// Registry credentials ID for private container registries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,
    /// If specified, overrides the ENTRYPOINT for the Docker image.
    /// If empty, uses the ENTRYPOINT defined in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,
    /// If specified, overrides the start CMD for the Docker image.
    /// If empty, uses the start CMD defined in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,
    /// Environment variables for the Pod container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,
    /// Set to true to enable global networking for the Pod.
    /// Currently only available for On-Demand GPU Pods on some Secure Cloud data centers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networking: Option<bool>,
    /// The image tag for the container run on the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// Set to true to lock the Pod. Locking a Pod disables stopping or resetting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// A user-defined name for the Pod. The name does not need to be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of ports exposed on the Pod. Each port is formatted as
    /// `[port number]/[protocol]`. Protocol can be either `http` or `tcp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    /// The amount of disk space, in gigabytes (GB), to allocate on the Pod volume.
    /// The data on the Pod volume is persisted across Pod restarts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    /// The absolute path where the network volume will be mounted in the filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Query parameters for filtering and configuring Pod list operations.
///
/// This struct provides various filters and options for customizing the
/// response when listing Pods.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::pod::ListPodsQuery;
/// use runpod_sdk::model::common::{ComputeType, PodStatus};
///
/// let query = ListPodsQuery {
///     compute_type: Some(ComputeType::Gpu),
///     desired_status: Some(PodStatus::Running),
///     include_machine: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPodsQuery {
    /// Filter to only GPU or only CPU Pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    /// Filter to CPU Pods with any of the listed CPU flavors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_id: Option<Vec<CpuFlavorId>>,
    /// Filter to Pods located in any of the provided RunPod data centers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_id: Option<Vec<DataCenterId>>,
    /// Filter to Pods currently in the provided state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<PodStatus>,
    /// Filter to workers on the provided Serverless endpoint.
    /// Note that endpoint workers are not included in the response by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Filter to Pods with any of the listed GPU types attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_id: Option<Vec<GpuTypeId>>,
    /// Filter to a specific Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Filter to Pods created with the provided image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// Include information about the machine the Pod is running on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_machine: Option<bool>,
    /// Include information about the network volume attached to the Pod, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_network_volume: Option<bool>,
    /// Include information about the savings plans applied to the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_savings_plans: Option<bool>,
    /// Include information about the template the Pod uses, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    /// Set to true to also list Pods which are Serverless workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
    /// Filter to Pods with the provided name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter to Pods with the provided network volume attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,
    /// Filter to Pods created from the provided template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// Query parameters for retrieving a single Pod.
///
/// This struct provides options for customizing the response when retrieving
/// a specific Pod by ID.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::pod::GetPodQuery;
///
/// let query = GetPodQuery {
///     include_machine: Some(true),
///     include_network_volume: Some(true),
///     include_savings_plans: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPodQuery {
    /// Include information about the machine the Pod is running on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_machine: Option<bool>,
    /// Include information about the network volume attached to the returned Pod, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_network_volume: Option<bool>,
    /// Include information about the savings plans applied to the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_savings_plans: Option<bool>,
    /// Include information about the template the Pod uses, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    /// Set to true to also list Pods which are Serverless workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}
