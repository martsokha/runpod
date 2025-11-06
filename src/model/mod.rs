//! RunPod API data models and request/response types.
//!
//! This module contains all the data structures used to interact with the RunPod API.
//!
//! All types are re-exported from internal modules for a clean public API. The module
//! contains models for:
//!
//! - **Billing**: Usage statistics and billing records
//! - **Common**: Shared types like compute configurations and hardware specs
//! - **Endpoints**: Serverless endpoint configuration and management
//! - **Pods**: Pod lifecycle, configuration, and status
//! - **Registry**: Container registry authentication
//! - **Templates**: Reusable deployment templates
//! - **Volumes**: Network volume configuration and management
//!
//! # Features
//!
//! - **Type Safety**: All models use strongly typed fields with validation
//! - **Serialization**: Full serde support for JSON serialization/deserialization
//! - **Builder Pattern**: Many models support builder patterns for easy construction
//! - **Default Values**: Sensible defaults for optional fields
//!
//! # Examples
//!
//! Creating a pod configuration:
//!
//! ```no_run
//! use runpod_sdk::model::{PodCreateInput, ComputeType, CloudType, GpuTypeId};
//!
//! let pod_config = PodCreateInput {
//!     name: Some("my-pod".to_string()),
//!     image_name: Some("runpod/pytorch:2.0.1-py3.10-cuda11.8.0-devel-ubuntu22.04".to_string()),
//!     gpu_type_ids: Some(vec![GpuTypeId::NvidiaGeForceRtx3070]),
//!     compute_type: Some(ComputeType::Gpu),
//!     cloud_type: Some(CloudType::Community),
//!     ..Default::default()
//! };
//! ```

mod billing;
mod common;
mod endpoint;
mod pod;
mod registry;
mod template;
mod volume;

pub use billing::{
    BillingGrouping, BillingRecord, BillingRecords, BucketSize, EndpointBillingQuery,
    NetworkVolumeBillingQuery, PodBillingQuery,
};
pub use common::{
    CloudType, ComputeType, CpuFlavorId, CpuType, CudaVersion, DataCenterId, EnvVars, GpuInfo,
    GpuTypeId, Machine, NetworkVolume, PodStatus, PortMappings, SavingsPlan,
};
pub use endpoint::{
    Endpoint, EndpointCreateInput, EndpointUpdateInput, Endpoints, GetEndpointQuery,
    ListEndpointsQuery, ScalerType,
};
pub use pod::{GetPodQuery, ListPodsQuery, Pod, PodCreateInput, PodUpdateInput, Pods};
pub use registry::{
    ContainerRegistryAuth, ContainerRegistryAuthCreateInput, ContainerRegistryAuths,
};
pub use template::{
    GetTemplateQuery, ListTemplatesQuery, Template, TemplateCategory, TemplateCreateInput,
    TemplateUpdateInput, Templates,
};
pub use volume::{NetworkVolumeCreateInput, NetworkVolumeUpdateInput, NetworkVolumes};
