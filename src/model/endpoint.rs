//! Serverless endpoint models and related types for the RunPod API.
//!
//! This module contains all the data structures and types needed to work with RunPod Serverless Endpoints,
//! which provide auto-scaling, on-demand compute infrastructure for AI/ML workloads and applications.
//!
//! # Overview
//!
//! Serverless endpoints provide scalable, event-driven compute that automatically manages infrastructure
//! based on request load. They offer:
//!
//! - **Auto-scaling**: Workers automatically spin up/down based on request queue and configured policies
//! - **Template-based Deployment**: Consistent environments using pre-configured templates
//! - **Multi-region Distribution**: Deploy across multiple data centers for global availability
//! - **Cost Optimization**: Pay only for compute time used, with idle scaling to minimize costs
//! - **High Availability**: Automatic failover and redundancy across available infrastructure
//! - **Flexible Compute Types**: Support for both GPU-accelerated and CPU-only workloads
//!
//! # Core Types
//!
//! - [`Endpoint`]: The main serverless endpoint resource with full configuration and status
//! - [`EndpointCreateInput`]: Parameters for creating new serverless endpoints
//! - [`EndpointUpdateInput`]: Parameters for updating existing endpoints (triggers rolling updates)
//! - [`ScalerType`]: Scaling strategy enumeration (queue delay vs request count based)
//! - [`ListEndpointsQuery`]: Query parameters for listing endpoints with optional includes
//! - [`GetEndpointQuery`]: Query parameters for retrieving individual endpoints
//! - [`Endpoints`]: Type alias for collections of endpoints
//!
//! # Endpoint Lifecycle
//!
//! 1. **Creation**: Define compute requirements, scaling policy, and deployment template
//! 2. **Deployment**: Workers are provisioned across specified data centers
//! 3. **Auto-scaling**: System responds to request load by scaling workers up/down
//! 4. **Management**: Update configuration, monitor performance, manage costs
//! 5. **Termination**: Delete when no longer needed to stop all billing
//!
//! # Scaling Strategies
//!
//! ## Queue Delay Scaling (`QUEUE_DELAY`)
//! - **Use case**: Latency-sensitive applications requiring predictable response times
//! - **Behavior**: Scales up when requests wait longer than `scaler_value` seconds
//! - **Best for**: Real-time inference, interactive applications, SLA requirements
//! - **Trade-off**: May maintain higher baseline costs to ensure responsiveness
//!
//! ## Request Count Scaling (`REQUEST_COUNT`)
//! - **Use case**: Throughput-oriented applications with flexible response times
//! - **Behavior**: Maintains `queue_size / scaler_value` workers to handle load
//! - **Best for**: Batch processing, background tasks, cost-optimized workloads
//! - **Trade-off**: May have higher latency spikes during traffic bursts
//!
//! # Compute Types
//!
//! ## GPU Endpoints
//! - **Hardware**: NVIDIA GPUs (RTX, Tesla, H100 series) for AI/ML acceleration
//! - **Use cases**: Machine learning inference, image/video processing, AI training
//! - **Configuration**: GPU type selection, CUDA version constraints, multi-GPU support
//! - **Performance**: High-throughput parallel processing with specialized AI frameworks
//!
//! ## CPU Endpoints
//! - **Hardware**: High-performance x86 CPUs with flexible vCPU allocation
//! - **Use cases**: Web APIs, data processing, general computation, microservices
//! - **Configuration**: CPU flavor selection, vCPU count, memory optimization
//! - **Cost**: Lower cost per hour, ideal for CPU-bound workloads
//!
//! # Template Integration
//!
//! Endpoints are deployed using templates that define:
//! - **Container Image**: Docker image with application code and dependencies
//! - **Environment**: Runtime configuration, environment variables, startup commands
//! - **Resources**: Compute requirements, storage allocation, network configuration
//! - **Ports**: Exposed services and networking setup
//!
//! # Performance Optimization
//!
//! ## Flash Boot
//! - **Purpose**: Dramatically reduce worker startup time (seconds vs minutes)
//! - **Mechanism**: Pre-warmed container images with cached dependencies
//! - **Trade-off**: Higher per-request cost for faster cold start performance
//! - **Best for**: Interactive applications, real-time inference, low-latency requirements
//!
//! ## Data Center Strategy
//! - **Global Distribution**: Deploy across multiple regions for user proximity
//! - **Availability**: Automatic failover between data centers during outages
//! - **Latency**: Choose regions close to your users and data sources
//! - **Compliance**: Consider data sovereignty and regulatory requirements
//!
//! # Cost Management
//!
//! - **Worker Minutes**: Billed for actual compute time while workers are running
//! - **Idle Timeout**: Automatic worker shutdown to minimize costs during low usage
//! - **Min Workers**: Reserved capacity that runs continuously (lower rate, always charged)
//! - **Max Workers**: Burst capacity limit to control maximum spend
//! - **Request-based Billing**: Only pay when workers are processing requests
//!
//! # Examples
//!
//! ```rust
//! use runpod_sdk::model::endpoint::{EndpointCreateInput, EndpointUpdateInput, ScalerType};
//! use runpod_sdk::model::common::{ComputeType, CudaVersion};
//!
//! // Create a high-performance GPU endpoint for real-time AI inference
//! let ai_endpoint = EndpointCreateInput {
//!     template_id: "my-pytorch-template".to_string(),
//!     name: Some("production-ai-inference".to_string()),
//!     compute_type: Some(ComputeType::Gpu),
//!     gpu_count: Some(1),
//!     gpu_type_ids: Some(vec!["NVIDIA A100 80GB PCIe".to_string()]),
//!     allowed_cuda_versions: Some(vec![CudaVersion::V12_1]),
//!     scaler_type: Some(ScalerType::QueueDelay),
//!     scaler_value: Some(2), // Scale up if requests wait > 2 seconds
//!     workers_min: Some(1),  // Always keep 1 worker warm
//!     workers_max: Some(10), // Burst up to 10 workers
//!     idle_timeout: Some(30), // Shutdown after 30s idle
//!     flashboot: Some(true), // Enable fast startup
//!     execution_timeout_ms: Some(300000), // 5 minute max execution
//!     ..Default::default()
//! };
//!
//! // Create a cost-optimized CPU endpoint for batch processing
//! let batch_endpoint = EndpointCreateInput {
//!     template_id: "data-processing-template".to_string(),
//!     name: Some("batch-data-processor".to_string()),
//!     compute_type: Some(ComputeType::Cpu),
//!     vcpu_count: Some(4),
//!     scaler_type: Some(ScalerType::RequestCount),
//!     scaler_value: Some(5), // 1 worker per 5 queued requests
//!     workers_min: Some(0),  // No reserved capacity
//!     workers_max: Some(50), // Large burst capacity
//!     idle_timeout: Some(60), // Aggressive idle shutdown
//!     flashboot: Some(false), // Standard startup (lower cost)
//!     execution_timeout_ms: Some(3600000), // 1 hour max execution
//!     ..Default::default()
//! };
//!
//! // Update endpoint to handle increased load
//! let scale_up_update = EndpointUpdateInput {
//!     workers_max: Some(20),  // Double max capacity
//!     scaler_value: Some(1),  // More aggressive scaling
//!     flashboot: Some(true),  // Enable fast startup
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

use super::common::*;
use super::pod::Pod;
use super::template::Template;

/// Scaling strategy for serverless endpoint worker management.
///
/// Determines how the serverless infrastructure responds to incoming request load
/// by automatically scaling the number of active workers up or down.
///
/// # Strategies
///
/// ## Queue Delay (`QueueDelay`)
/// **Latency-optimized scaling** that prioritizes response time consistency.
/// - Scales up when requests wait longer than the configured threshold
/// - Maintains responsive service with predictable latency characteristics
/// - Best for interactive applications, real-time inference, SLA-sensitive workloads
/// - Higher baseline costs to ensure responsiveness
///
/// ## Request Count (`RequestCount`)
/// **Throughput-optimized scaling** that balances load efficiently across workers.
/// - Maintains approximately `queue_size / scaler_value` workers
/// - Optimizes for cost efficiency and overall throughput
/// - Best for batch processing, background tasks, cost-sensitive workloads
/// - May have higher latency during traffic spikes
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::ScalerType;
///
/// // For real-time AI inference requiring <3s response times
/// let latency_optimized = ScalerType::QueueDelay;
/// // scaler_value = 2 means scale up if any request waits >2 seconds
///
/// // For batch image processing where cost matters more than speed
/// let cost_optimized = ScalerType::RequestCount;
/// // scaler_value = 10 means maintain 1 worker per 10 queued requests
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "strum", strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
pub enum ScalerType {
    /// Queue delay-based scaling - prioritizes response time consistency.
    /// Scales up when requests wait longer than the threshold.
    QueueDelay,
    /// Request count-based scaling - optimizes for throughput and cost.
    /// Maintains workers proportional to queue depth.
    RequestCount,
}

/// Serverless endpoint resource providing auto-scaling compute infrastructure.
///
/// Represents a fully configured serverless endpoint with all deployment settings,
/// scaling configuration, and runtime status. Endpoints automatically manage
/// worker lifecycle based on request load and configured policies.
///
/// # Key Properties
///
/// - **Auto-scaling**: Workers spin up/down based on request queue and scaling policy
/// - **Template-driven**: Consistent runtime environment from pre-configured templates
/// - **Multi-region**: Distributed deployment across multiple data centers
/// - **Cost-optimized**: Pay-per-use billing with idle timeout management
/// - **High-availability**: Automatic failover and redundancy
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::Endpoint;
///
/// // Endpoint instances are typically obtained from API responses
/// // when listing, creating, or retrieving serverless endpoints
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    /// A unique string identifying the serverless endpoint.
    pub id: String,

    /// A user-defined name for the endpoint. The name does not need to be unique.
    ///
    /// Used for organization and identification in dashboards and monitoring.
    /// Can be updated without affecting endpoint functionality.
    pub name: Option<String>,

    /// A unique string identifying the RunPod user who created the endpoint.
    pub user_id: String,

    /// The unique string identifying the template used to create the endpoint.
    ///
    /// Templates define the container image, environment, and resource configuration
    /// that will be deployed across all workers for this endpoint.
    pub template_id: String,

    /// The current version of the endpoint configuration.
    ///
    /// Incremented whenever the template or environment variables are changed,
    /// triggering a rolling update of all workers.
    pub version: i32,

    /// The type of compute used by workers on this endpoint.
    ///
    /// Determines whether workers will have GPU or CPU compute resources attached.
    /// This setting affects pricing, available hardware types, and performance characteristics.
    pub compute_type: ComputeType,

    /// The UTC timestamp when the endpoint was created.
    ///
    /// ISO 8601 format string representing the endpoint creation time.
    pub created_at: String,

    /// List of RunPod data center IDs where workers can be located.
    ///
    /// Workers are distributed across these data centers for availability and performance.
    /// The system automatically selects the best available data center based on
    /// resource availability and proximity to users.
    pub data_center_ids: Vec<DataCenterId>,

    /// Environment variables for the endpoint's container runtime.
    ///
    /// These variables are injected into all worker containers and can be used
    /// for configuration, API keys, feature flags, and other runtime settings.
    pub env: Option<EnvVars>,

    /// The maximum execution time in milliseconds for individual requests.
    ///
    /// If a request exceeds this timeout, the worker is stopped and the request
    /// is marked as failed. This prevents runaway processes and ensures
    /// predictable resource usage.
    ///
    /// **Common values:**
    /// - Web APIs: 30,000ms (30 seconds)
    /// - AI inference: 300,000ms (5 minutes)
    /// - Batch processing: 3,600,000ms (1 hour)
    pub execution_timeout_ms: i32,

    /// The number of GPUs attached to each worker (GPU endpoints only).
    ///
    /// Only relevant when `compute_type` is `GPU`. Determines the GPU resources
    /// allocated to each worker instance for parallel processing workloads.
    pub gpu_count: Option<i32>,

    /// List of RunPod GPU types that can be attached to workers (GPU endpoints only).
    ///
    /// The system tries to allocate GPUs in the order specified, falling back
    /// to subsequent types if the preferred options are unavailable.
    /// Only relevant when `compute_type` is `GPU`.
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,

    /// List of CPU instance IDs that can be attached to workers (CPU endpoints only).
    ///
    /// For CPU endpoints, specifies the available instance types that workers
    /// can use, allowing the system to choose based on availability and cost.
    pub instance_ids: Option<Vec<String>>,

    /// The number of seconds a worker can be idle before being scaled down.
    ///
    /// Workers that haven't processed requests for this duration are automatically
    /// terminated to reduce costs. Shorter timeouts reduce costs but may increase
    /// cold start latency for subsequent requests.
    ///
    /// **Typical values:**
    /// - Cost-optimized: 30-60 seconds
    /// - Balanced: 5-15 seconds
    /// - Performance-optimized: 1-5 seconds
    pub idle_timeout: i32,

    /// The unique ID of the network volume attached to workers, if any.
    ///
    /// Network volumes provide persistent, shared storage across all workers,
    /// useful for model weights, datasets, and other shared assets.
    pub network_volume_id: Option<String>,

    /// The scaling strategy used to manage worker count.
    ///
    /// Determines how the system responds to request load by scaling workers
    /// up or down automatically.
    pub scaler_type: ScalerType,

    /// The scaling sensitivity parameter.
    ///
    /// **For `QueueDelay` scaling:**
    /// - Seconds a request can wait in queue before scaling up
    /// - Lower values = more responsive but potentially higher costs
    ///
    /// **For `RequestCount` scaling:**
    /// - Target requests per worker (queue_size / scaler_value = worker_count)
    /// - Higher values = fewer workers, more cost-efficient
    pub scaler_value: i32,

    /// The maximum number of workers that can run simultaneously.
    ///
    /// Hard limit preventing runaway scaling and controlling maximum costs.
    /// Set based on expected peak load and budget constraints.
    pub workers_max: i32,

    /// The minimum number of workers that always remain running.
    ///
    /// Reserved capacity that's always available, even during idle periods.
    /// These workers are billed at a lower rate but provide immediate availability.
    /// Set to 0 for maximum cost efficiency, or >0 for better responsiveness.
    pub workers_min: i32,

    /// List of acceptable CUDA versions for GPU workers.
    ///
    /// If specified, only workers with compatible CUDA runtimes will be used.
    /// Useful for ensuring compatibility with specific AI/ML frameworks.
    /// Only relevant for GPU endpoints.
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,

    /// Detailed template information (included when `include_template` is true).
    ///
    /// Contains the full template configuration including container image,
    /// environment setup, and resource requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,

    /// Current worker instances (included when `include_workers` is true).
    ///
    /// List of active worker pods with their current status, resource allocation,
    /// and performance metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<Vec<Pod>>,
}

/// List of serverless endpoints.
///
/// A collection type representing multiple endpoints, typically returned
/// from API endpoints that list endpoints for an account.
pub type Endpoints = Vec<Endpoint>;

/// Input parameters for creating a new serverless endpoint.
///
/// This struct contains all the configuration options available when creating an endpoint,
/// including compute specifications, scaling policies, and deployment preferences.
/// Most fields are optional and will use RunPod defaults if not specified.
///
/// # Required Fields
///
/// Only `template_id` is required - all other configuration uses sensible defaults
/// that can be customized based on your specific workload requirements.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::{EndpointCreateInput, ScalerType};
/// use runpod_sdk::model::common::{ComputeType, CudaVersion};
///
/// // High-performance GPU endpoint for real-time AI inference
/// let inference_endpoint = EndpointCreateInput {
///     template_id: "pytorch-inference-template".to_string(),
///     name: Some("ai-inference-prod".to_string()),
///     compute_type: Some(ComputeType::Gpu),
///     gpu_count: Some(1),
///     gpu_type_ids: Some(vec!["NVIDIA A100 80GB PCIe".to_string()]),
///     allowed_cuda_versions: Some(vec![CudaVersion::V12_1]),
///     scaler_type: Some(ScalerType::QueueDelay),
///     scaler_value: Some(3), // Scale if requests wait >3 seconds
///     workers_min: Some(1),  // Keep 1 worker always ready
///     workers_max: Some(5),  // Burst up to 5 workers
///     flashboot: Some(true), // Fast cold starts
///     idle_timeout: Some(30), // Scale down after 30s idle
///     execution_timeout_ms: Some(300000), // 5 minute timeout
///     ..Default::default()
/// };
///
/// // Cost-optimized CPU endpoint for batch processing
/// let batch_endpoint = EndpointCreateInput {
///     template_id: "batch-processor-template".to_string(),
///     name: Some("data-batch-processor".to_string()),
///     compute_type: Some(ComputeType::Cpu),
///     vcpu_count: Some(8),
///     scaler_type: Some(ScalerType::RequestCount),
///     scaler_value: Some(10), // 1 worker per 10 requests
///     workers_min: Some(0),   // No reserved capacity
///     workers_max: Some(20),  // Allow large bursts
///     flashboot: Some(false), // Standard startup (cheaper)
///     idle_timeout: Some(120), // Longer idle time for batches
///     execution_timeout_ms: Some(1800000), // 30 minute timeout
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointCreateInput {
    /// The unique string identifying the template used to create the endpoint.
    ///
    /// **Required field** - specifies the container image, environment, and
    /// resource configuration that will be deployed across all workers.
    ///
    /// Templates ensure consistent runtime environments and can be shared
    /// across multiple endpoints for standardized deployments.
    pub template_id: String,

    /// If the endpoint is a GPU endpoint, acceptable CUDA versions for workers.
    ///
    /// Constrains worker allocation to machines with compatible CUDA runtimes.
    /// Useful for ensuring compatibility with specific AI/ML framework versions
    /// that require particular CUDA versions.
    ///
    /// **Default**: Any CUDA version is acceptable
    /// **GPU endpoints only**: Ignored for CPU endpoints
    ///
    /// **Example**: `[CudaVersion::V12_1, CudaVersion::V11_8]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,

    /// Set to `GPU` for GPU-accelerated workers, `CPU` for CPU-only workers.
    ///
    /// Determines the type of compute resources allocated to workers:
    /// - `GPU`: Workers get GPU acceleration for AI/ML workloads
    /// - `CPU`: Workers get high-performance CPUs for general compute
    ///
    /// **Default**: `GPU`
    /// **Impact**: Affects available hardware types, pricing, and performance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,

    /// If creating a CPU endpoint, list of CPU flavors for workers.
    ///
    /// Specifies the CPU configurations that can be used for workers.
    /// The order determines rental priority - preferred flavors first.
    ///
    /// **CPU endpoints only**: Ignored for GPU endpoints
    /// **Default**: All available CPU flavors
    ///
    /// **Available flavors**: `cpu3c`, `cpu3g`, `cpu5c`, `cpu5g`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,

    /// List of data center IDs where workers can be located.
    ///
    /// Workers are distributed across these data centers for availability,
    /// performance, and proximity to users. The system automatically
    /// selects the best available data center for each worker.
    ///
    /// **Default**: All available data centers globally
    /// **Strategy**: Choose data centers close to your users and data sources
    ///
    /// **Common choices:**
    /// - Global: `["US-CA-1", "EU-RO-1", "AP-JP-1"]`
    /// - Regional: `["US-TX-1", "US-CA-2"]` for US-only
    /// - Single DC: `["EU-RO-1"]` for data residency requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,

    /// Maximum execution time in milliseconds for individual requests.
    ///
    /// Requests exceeding this timeout are terminated and marked as failed.
    /// Prevents runaway processes and ensures predictable resource usage.
    ///
    /// **Default**: 600,000ms (10 minutes)
    /// **Range**: 1,000ms to 3,600,000ms (1 second to 1 hour)
    ///
    /// **Guidelines:**
    /// - Web APIs: 30,000ms (30 seconds)
    /// - AI inference: 300,000ms (5 minutes)
    /// - Image processing: 600,000ms (10 minutes)
    /// - Batch jobs: 3,600,000ms (1 hour)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_ms: Option<i32>,

    /// Whether to enable flash boot for faster worker startup.
    ///
    /// Flash boot dramatically reduces cold start time by using pre-warmed
    /// container images with cached dependencies and optimized initialization.
    ///
    /// **Default**: `false`
    /// **Trade-off**: Higher per-request cost for much faster startup
    /// **Best for**: Interactive applications, real-time inference, low-latency requirements
    /// **Startup time**: ~5-10 seconds with flash boot vs 30-60 seconds without
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashboot: Option<bool>,

    /// If creating a GPU endpoint, number of GPUs per worker.
    ///
    /// Determines GPU resources allocated to each worker for parallel processing.
    /// More GPUs enable larger models and higher throughput but increase costs.
    ///
    /// **Default**: 1
    /// **GPU endpoints only**: Ignored for CPU endpoints
    /// **Range**: 1-8 depending on GPU type availability
    ///
    /// **Use cases:**
    /// - Single GPU: Most inference workloads, small models
    /// - Multi-GPU: Large language models, distributed training, high-throughput inference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,

    /// If creating a GPU endpoint, list of GPU types for workers.
    ///
    /// Specifies GPU hardware that can be used for workers. The order
    /// determines rental priority - the system tries preferred types first.
    ///
    /// **GPU endpoints only**: Ignored for CPU endpoints
    /// **Default**: All available GPU types
    ///
    /// **Performance tiers:**
    /// - High-end: `"NVIDIA H100 80GB HBM3"`, `"NVIDIA A100 80GB PCIe"`
    /// - Mid-range: `"NVIDIA RTX A6000"`, `"NVIDIA A40"`
    /// - Budget: `"NVIDIA RTX 4090"`, `"NVIDIA RTX 3090"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,

    /// Number of seconds workers can be idle before scaling down.
    ///
    /// Workers that haven't processed requests for this duration are
    /// automatically terminated to reduce costs. Balance between cost
    /// optimization and cold start latency.
    ///
    /// **Default**: 5 seconds
    /// **Range**: 1-3600 seconds (1 second to 1 hour)
    ///
    /// **Strategy:**
    /// - Aggressive (cost-focused): 30-60 seconds
    /// - Balanced: 5-15 seconds
    /// - Responsive (latency-focused): 1-5 seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,

    /// A user-defined name for the endpoint.
    ///
    /// Used for organization and identification in dashboards, monitoring,
    /// and API responses. The name does not need to be unique across your account.
    ///
    /// **Default**: Auto-generated based on template name
    /// **Max length**: 191 characters
    /// **Best practices**: Use descriptive names like "prod-image-classifier" or "staging-api-v2"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The unique ID of a network volume to attach to workers.
    ///
    /// Network volumes provide persistent, shared storage across all workers,
    /// useful for model weights, datasets, cached data, and other shared assets.
    ///
    /// **Default**: No network volume attached
    /// **Requirements**: Volume must exist in same data centers as workers
    /// **Use cases**: Model storage, dataset access, shared caching, persistent logs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,

    /// The scaling strategy for managing worker count.
    ///
    /// Determines how the system automatically scales workers up/down based
    /// on request load and queue depth.
    ///
    /// **Default**: `QueueDelay`
    ///
    /// **Strategies:**
    /// - `QueueDelay`: Scale based on request wait time (latency-optimized)
    /// - `RequestCount`: Scale based on queue depth (throughput-optimized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_type: Option<ScalerType>,

    /// The scaling sensitivity parameter.
    ///
    /// Meaning depends on the `scaler_type`:
    ///
    /// **For `QueueDelay`**: Maximum seconds requests can wait before scaling up
    /// - Lower values = more responsive scaling, higher costs
    /// - Higher values = slower scaling, lower costs
    ///
    /// **For `RequestCount`**: Target requests per worker
    /// - `queue_size / scaler_value = target_worker_count`
    /// - Lower values = more workers, lower latency
    /// - Higher values = fewer workers, higher latency
    ///
    /// **Default**: 4
    /// **Range**: 1-3600
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_value: Option<i32>,

    /// If creating a CPU endpoint, number of vCPUs per worker.
    ///
    /// Determines CPU resources allocated to each worker. More vCPUs enable
    /// higher parallelism and throughput for CPU-intensive workloads.
    ///
    /// **Default**: 2 vCPUs
    /// **CPU endpoints only**: Ignored for GPU endpoints
    /// **Range**: 1-32 vCPUs depending on CPU flavor
    ///
    /// **Guidelines:**
    /// - Light workloads: 1-2 vCPUs
    /// - Web APIs: 2-4 vCPUs
    /// - Data processing: 4-16 vCPUs
    /// - Heavy computation: 16+ vCPUs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,

    /// Maximum number of workers that can run simultaneously.
    ///
    /// Hard limit preventing runaway scaling and controlling maximum costs.
    /// Set based on expected peak load, budget constraints, and infrastructure limits.
    ///
    /// **Default**: No limit (subject to account quotas)
    /// **Range**: 0-1000+ depending on account limits
    ///
    /// **Strategy**: Set 2-3x expected peak load for safety margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_max: Option<i32>,

    /// Minimum number of workers that always remain running.
    ///
    /// Reserved capacity providing immediate availability even during idle
    /// periods. These workers are billed at a reduced rate but ensure
    /// zero cold start latency for the first few requests.
    ///
    /// **Default**: 0 (no reserved capacity)
    /// **Range**: 0-100 depending on account limits
    ///
    /// **Trade-offs:**
    /// - 0: Maximum cost efficiency, but cold starts for first requests
    /// - 1+: Immediate availability, continuous billing for reserved workers
    ///
    /// **Strategy**: Set to 1 for production endpoints requiring <1s response time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_min: Option<i32>,
}

/// Input parameters for updating an existing serverless endpoint.
///
/// This struct allows you to modify endpoint configuration and trigger a rolling
/// release that updates all workers with the new settings. All fields are optional,
/// allowing you to update only the properties you want to change.
///
/// # Rolling Release Process
///
/// When an endpoint is updated:
/// 1. **Validation**: New configuration is validated for compatibility
/// 2. **Version Increment**: Endpoint version number is incremented
/// 3. **Rolling Update**: Workers are gradually replaced with new configuration
/// 4. **Traffic Migration**: Requests are routed to updated workers as they become available
/// 5. **Cleanup**: Old workers are terminated once traffic migration is complete
///
/// # Important Notes
///
/// - **Zero Downtime**: Updates are performed without service interruption
/// - **Gradual Rollout**: Workers are updated in batches to maintain availability
/// - **Rollback**: Previous versions can be restored if issues are detected
/// - **Template Changes**: Updating `template_id` deploys new container images
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::{EndpointUpdateInput, ScalerType};
///
/// // Scale up for increased traffic
/// let scale_up = EndpointUpdateInput {
///     workers_max: Some(20),      // Double capacity
///     scaler_value: Some(2),      // More aggressive scaling
///     idle_timeout: Some(10),     // Keep workers longer
///     ..Default::default()
/// };
///
/// // Enable flash boot for better performance
/// let performance_upgrade = EndpointUpdateInput {
///     flashboot: Some(true),
///     execution_timeout_ms: Some(60000), // Reduce timeout
///     ..Default::default()
/// };
///
/// // Switch to cost-optimized scaling
/// let cost_optimization = EndpointUpdateInput {
///     scaler_type: Some(ScalerType::RequestCount),
///     scaler_value: Some(10),     // 1 worker per 10 requests
///     workers_min: Some(0),       // No reserved capacity
///     flashboot: Some(false),     // Standard startup
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointUpdateInput {
    /// If the endpoint is a GPU endpoint, acceptable CUDA versions for workers.
    ///
    /// Updates the CUDA version constraints for worker allocation.
    /// Triggers rolling release to ensure all workers use compatible CUDA versions.
    ///
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cuda_versions: Option<Vec<CudaVersion>>,

    /// If the endpoint is a CPU endpoint, list of CPU flavors for workers.
    ///
    /// Updates the available CPU configurations for workers.
    /// The order determines rental priority for new workers.
    ///
    /// **CPU endpoints only**: Ignored for GPU endpoints
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_flavor_ids: Option<Vec<CpuFlavorId>>,

    /// List of data center IDs where workers can be located.
    ///
    /// Updates the geographic distribution of workers.
    /// Existing workers in removed data centers will be gradually replaced.
    ///
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_ids: Option<Vec<DataCenterId>>,

    /// Maximum execution time in milliseconds for individual requests.
    ///
    /// Updates the timeout for request processing. Affects new requests
    /// immediately, existing requests continue with previous timeout.
    ///
    /// **Range**: 1,000ms to 3,600,000ms (1 second to 1 hour)
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_ms: Option<i32>,

    /// Whether to enable flash boot for faster worker startup.
    ///
    /// Updates the startup optimization for new workers.
    /// Affects cold start performance and per-request costs.
    ///
    /// **Trade-off**: Higher per-request cost for faster startup
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashboot: Option<bool>,

    /// If the endpoint is a GPU endpoint, number of GPUs per worker.
    ///
    /// Updates GPU allocation for new workers. Triggers rolling release
    /// to deploy workers with the new GPU configuration.
    ///
    /// **GPU endpoints only**: Ignored for CPU endpoints
    /// **Range**: 1-8 depending on GPU type availability
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_count: Option<i32>,

    /// If the endpoint is a GPU endpoint, list of GPU types for workers.
    ///
    /// Updates available GPU hardware types for workers.
    /// The order determines rental priority for new workers.
    ///
    /// **GPU endpoints only**: Ignored for CPU endpoints
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_type_ids: Option<Vec<GpuTypeId>>,

    /// Number of seconds workers can be idle before scaling down.
    ///
    /// Updates the idle timeout for worker lifecycle management.
    /// Affects cost optimization and cold start frequency.
    ///
    /// **Range**: 1-3600 seconds (1 second to 1 hour)
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,

    /// A user-defined name for the endpoint.
    ///
    /// Updates the display name used in dashboards and API responses.
    /// This change is applied immediately without triggering a rolling release.
    ///
    /// **Max length**: 191 characters
    /// **Note**: Set to `None` to keep current name unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The unique ID of a network volume to attach to workers.
    ///
    /// Updates the persistent storage attached to workers.
    /// Triggers rolling release to mount/unmount volumes on all workers.
    ///
    /// **Requirements**: Volume must exist in same data centers as workers
    /// **Note**: Set to `None` to keep current volume unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_volume_id: Option<String>,

    /// The scaling strategy for managing worker count.
    ///
    /// Updates the auto-scaling algorithm used for worker management.
    /// Change takes effect immediately for new scaling decisions.
    ///
    /// **Strategies:**
    /// - `QueueDelay`: Scale based on request wait time
    /// - `RequestCount`: Scale based on queue depth
    /// **Note**: Set to `None` to keep current strategy unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_type: Option<ScalerType>,

    /// The scaling sensitivity parameter.
    ///
    /// Updates the scaling behavior sensitivity.
    /// Change takes effect immediately for new scaling decisions.
    ///
    /// **For QueueDelay**: Maximum seconds requests can wait
    /// **For RequestCount**: Target requests per worker
    /// **Range**: 1-3600
    /// **Note**: Set to `None` to keep current value unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_value: Option<i32>,

    /// The unique ID of the template used to create the endpoint.
    ///
    /// Updates the container image and environment configuration.
    /// Triggers rolling release to deploy all workers with the new template.
    ///
    /// **Impact**: Changes container image, environment, resource allocation
    /// **Rolling Release**: All workers are gradually replaced
    /// **Note**: Set to `None` to keep current template unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// If the endpoint is a CPU endpoint, number of vCPUs per worker.
    ///
    /// Updates CPU allocation for new workers. Triggers rolling release
    /// to deploy workers with the new CPU configuration.
    ///
    /// **CPU endpoints only**: Ignored for GPU endpoints
    /// **Range**: 1-32 vCPUs depending on CPU flavor
    /// **Note**: Set to `None` to keep current setting unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,

    /// Maximum number of workers that can run simultaneously.
    ///
    /// Updates the scaling limit for worker count.
    /// Change takes effect immediately for new scaling decisions.
    ///
    /// **Range**: 0-1000+ depending on account limits
    /// **Note**: Set to `None` to keep current limit unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_max: Option<i32>,

    /// Minimum number of workers that always remain running.
    ///
    /// Updates the reserved capacity for immediate availability.
    /// Change triggers immediate scaling to meet the new minimum.
    ///
    /// **Range**: 0-100 depending on account limits
    /// **Billing**: Reserved workers are always charged (at reduced rate)
    /// **Note**: Set to `None` to keep current minimum unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers_min: Option<i32>,
}

/// Query parameters for listing serverless endpoints.
///
/// Controls which additional data is included in the response when retrieving
/// multiple endpoints. Including additional data provides more detailed information
/// but increases response size and latency.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::ListEndpointsQuery;
///
/// // Basic listing (endpoints only)
/// let basic_query = ListEndpointsQuery::default();
///
/// // Include template details for each endpoint
/// let with_templates = ListEndpointsQuery {
///     include_template: Some(true),
///     include_workers: Some(false),
/// };
///
/// // Include both template and worker information
/// let full_details = ListEndpointsQuery {
///     include_template: Some(true),
///     include_workers: Some(true),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEndpointsQuery {
    /// Whether to include template information for each endpoint.
    ///
    /// When `true`, the response includes detailed template configuration
    /// including container image, environment variables, resource requirements,
    /// and deployment settings for each endpoint.
    ///
    /// **Default**: `false`
    /// **Impact**: Increases response size and latency
    /// **Useful for**: Deployment auditing, configuration comparison, debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,

    /// Whether to include current worker information for each endpoint.
    ///
    /// When `true`, the response includes detailed information about active
    /// workers including their status, resource allocation, performance metrics,
    /// and current workload for each endpoint.
    ///
    /// **Default**: `false`
    /// **Impact**: Significantly increases response size and latency
    /// **Useful for**: Capacity monitoring, performance analysis, troubleshooting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}

/// Query parameters for retrieving a single serverless endpoint.
///
/// Controls which additional data is included in the response when retrieving
/// a specific endpoint. Including additional data provides more detailed information
/// but increases response size and latency.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::endpoint::GetEndpointQuery;
///
/// // Basic endpoint information only
/// let basic_query = GetEndpointQuery::default();
///
/// // Include template configuration
/// let with_template = GetEndpointQuery {
///     include_template: Some(true),
///     include_workers: Some(false),
/// };
///
/// // Include complete details for monitoring
/// let monitoring_query = GetEndpointQuery {
///     include_template: Some(true),
///     include_workers: Some(true),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEndpointQuery {
    /// Whether to include template information in the response.
    ///
    /// When `true`, the response includes detailed template configuration
    /// including container image, environment variables, resource requirements,
    /// and deployment settings.
    ///
    /// **Default**: `false`
    /// **Impact**: Increases response size and latency
    /// **Useful for**: Configuration review, deployment debugging, audit trails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,

    /// Whether to include current worker information in the response.
    ///
    /// When `true`, the response includes detailed information about active
    /// workers including their status, resource allocation, performance metrics,
    /// machine details, and current workload.
    ///
    /// **Default**: `false`
    /// **Impact**: Significantly increases response size and latency
    /// **Useful for**: Real-time monitoring, performance optimization, troubleshooting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workers: Option<bool>,
}
