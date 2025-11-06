use serde::{Deserialize, Serialize};

use super::common::*;

/// Compute category classification for templates.
///
/// Templates are categorized based on their primary compute requirements,
/// which helps with resource allocation, pricing, and filtering in the RunPod ecosystem.
///
/// # Categories
///
/// - **NVIDIA**: Templates optimized for NVIDIA GPU acceleration
/// - **AMD**: Templates optimized for AMD GPU acceleration
/// - **CPU**: Templates for CPU-only compute workloads
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TemplateCategory {
    /// NVIDIA GPU-accelerated compute template.
    /// Optimized for workloads requiring NVIDIA CUDA capabilities,
    /// such as deep learning, scientific computing, and graphics processing.
    #[serde(rename = "NVIDIA")]
    #[default]
    Nvidia,

    /// AMD GPU-accelerated compute template.
    /// Optimized for workloads utilizing AMD GPU acceleration,
    /// including ROCm-based machine learning and compute applications.
    #[serde(rename = "AMD")]
    Amd,

    /// CPU-only compute template.
    /// Designed for general-purpose computing tasks that don't require
    /// GPU acceleration, such as web services, data processing, and development.
    #[serde(rename = "CPU")]
    Cpu,
}

/// Template resource containing deployment configuration and metadata.
///
/// A Template is a reusable configuration that defines how Pods and Serverless endpoints
/// should be deployed, including Docker image, resource requirements, environment variables,
/// and networking settings.
///
/// # Template Lifecycle
///
/// 1. **Creation**: Define template with [`TemplateCreateInput`]
/// 2. **Usage**: Reference template when creating Pods or Serverless endpoints
/// 3. **Updates**: Modify template with [`TemplateUpdateInput`] (triggers rolling releases)
/// 4. **Sharing**: Make public to share with community and earn revenue
/// 5. **Management**: Track usage statistics and earnings through template metadata
///
/// # Revenue Model
///
/// Public templates can generate revenue for their creators:
///
/// - Users pay standard RunPod rates for compute resources
/// - Template creators earn a percentage of compute costs
/// - Earnings are tracked in the `earned` field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    /// A unique string identifying the template.
    /// This ID is used to reference the template when creating Pods or Serverless endpoints.
    pub id: String,

    /// A user-defined name for the template.
    /// The name must be unique within your account and should be descriptive
    /// of the template's purpose or use case.
    pub name: String,

    /// The Docker image tag for containers deployed from this template.
    /// Can reference public images from Docker Hub, private registry images,
    /// or images from other container registries with appropriate authentication.
    pub image_name: String,

    /// Whether the template is visible to other RunPod users.
    /// - `true`: Template is public and can be used by any RunPod user
    /// - `false`: Template is private and only accessible to the creator
    pub is_public: bool,

    /// Whether this is an official template managed by RunPod.
    /// Official templates are curated, maintained, and optimized by the RunPod team
    /// for popular frameworks and use cases.
    pub is_runpod: bool,

    /// Whether the template is designed for Serverless workers or Pods.
    /// - `true`: Template creates Serverless workers (auto-scaling, event-driven)
    /// - `false`: Template creates Pods (persistent, long-running instances)
    pub is_serverless: bool,

    /// The compute category of resources defined by this template.
    /// Determines the type of hardware acceleration available to deployed instances.
    pub category: Option<TemplateCategory>,

    /// The amount of disk space, in gigabytes (GB), allocated on the container disk.
    /// Container disk provides fast local storage but data is wiped when instances restart.
    /// Use for temporary files, caches, and application data that doesn't need persistence.
    pub container_disk_in_gb: i32,

    /// The unique identifier for container registry authentication.
    /// Required when the template uses private Docker images that need authentication
    /// to pull from protected container registries.
    pub container_registry_auth_id: Option<String>,

    /// Override for the Docker image ENTRYPOINT.
    /// If specified, replaces the ENTRYPOINT defined in the Docker image.
    /// If `None` or empty, uses the ENTRYPOINT from the Docker image.
    pub docker_entrypoint: Option<Vec<String>>,

    /// Override for the Docker image start command.
    /// If specified, replaces the CMD defined in the Docker image.
    /// If `None` or empty, uses the CMD from the Docker image.
    pub docker_start_cmd: Option<Vec<String>>,

    /// The total RunPod credits earned by the template creator.
    /// Represents cumulative earnings from all Pods and Serverless workers
    /// created from this template by other users. Only applies to public templates.
    pub earned: Option<f64>,

    /// Environment variables to be set in containers deployed from this template.
    /// These variables are available to the running application and can be used
    /// for configuration, secrets management, and runtime customization.
    pub env: Option<EnvVars>,

    /// A list of ports exposed on deployed Pods or Serverless workers.
    /// Each port is formatted as `[port number]/[protocol]` where protocol
    /// can be either `http` or `tcp`. Example: `["8888/http", "22/tcp"]`
    pub ports: Option<Vec<String>>,

    /// Markdown-formatted documentation for the template.
    /// Displayed in the RunPod UI when users browse or select templates.
    /// Should include usage instructions, requirements, and configuration details.
    pub readme: Option<String>,

    /// Runtime statistics for the template (in minutes).
    /// Tracks total runtime across all instances deployed from this template.
    /// Used for usage analytics and optimization insights.
    pub runtime_in_min: Option<i32>,

    /// The amount of disk space, in gigabytes (GB), allocated on the local Pod volume.
    /// Pod volume provides persistent local storage that survives instance restarts.
    /// Use for data, models, and files that need to persist across restarts.
    pub volume_in_gb: i32,

    /// The absolute filesystem path where the Pod volume is mounted.
    /// This is where the persistent storage will be accessible within the container.
    /// Common paths include `/workspace`, `/data`, or `/app/storage`.
    pub volume_mount_path: String,
}

/// Collection of template records.
///
/// This type alias represents the standard response format when listing templates,
/// containing an array of [`Template`] instances with applied filters and access controls.
pub type Templates = Vec<Template>;

/// Input parameters for creating new templates.
///
/// Use this struct to define all aspects of a new template, from basic identification
/// to detailed runtime configuration. All fields except `name` and `image_name` are optional,
/// allowing for flexible template creation with sensible defaults.
///
/// # Required Fields
///
/// - `name`: Unique, descriptive template name
/// - `image_name`: Docker image for container deployment
///
/// # Default Behavior
///
/// When optional fields are omitted:
/// - `category`: Defaults to `TemplateCategory::Nvidia`
/// - `container_disk_in_gb`: Defaults to 50 GB
/// - `volume_in_gb`: Defaults to 20 GB
/// - `volume_mount_path`: Defaults to "/workspace"
/// - `is_public`: Defaults to `false` (private template)
/// - `is_serverless`: Defaults to `false` (Pod template)
/// - `ports`: Defaults to `["8888/http", "22/tcp"]`
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::{TemplateCreateInput, TemplateCategory};
///
/// let basic_template = TemplateCreateInput {
///     name: "My Basic Template".to_string(),
///     image_name: "python:3.9".to_string(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCreateInput {
    /// A user-defined name for the template.
    /// Must be unique within your account. Choose a descriptive name that
    /// clearly identifies the template's purpose or use case.
    pub name: String,

    /// The Docker image tag for containers deployed from this template.
    /// Can be a public image (e.g., "python:3.9") or private image with
    /// appropriate `container_registry_auth_id` for authentication.
    pub image_name: String,

    /// The compute category for this template.
    /// Determines the type of hardware acceleration available.
    /// Defaults to `TemplateCategory::Nvidia` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TemplateCategory>,

    /// The amount of disk space in GB to allocate for the container disk.
    /// Container disk provides fast local storage but data is wiped on restart.
    /// Defaults to 50 GB if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,

    /// The unique identifier for container registry authentication.
    /// Required when using private Docker images that need authentication.
    /// Reference the ID from a previously created container registry auth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,

    /// Override for the Docker image ENTRYPOINT.
    /// If specified, replaces the ENTRYPOINT defined in the Docker image.
    /// Leave empty to use the image's default ENTRYPOINT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,

    /// Override for the Docker image start command.
    /// If specified, replaces the CMD defined in the Docker image.
    /// Leave empty to use the image's default CMD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,

    /// Environment variables for containers deployed from this template.
    /// These variables are set in the container environment and available
    /// to the running application for configuration and customization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,

    /// Whether the template should be visible to other RunPod users.
    /// - `true`: Template is public and can generate revenue when used by others
    /// - `false`: Template is private and only accessible to you (default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// Whether the template is designed for Serverless workers or Pods.
    /// - `true`: Creates auto-scaling Serverless workers for event-driven workloads
    /// - `false`: Creates persistent Pods for long-running workloads (default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_serverless: Option<bool>,

    /// A list of ports to expose on deployed instances.
    /// Each port is formatted as `[port]/[protocol]` (e.g., "8888/http", "22/tcp").
    /// Defaults to `["8888/http", "22/tcp"]` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,

    /// Markdown-formatted documentation describing the template.
    /// Displayed in the RunPod UI and should include usage instructions,
    /// requirements, configuration details, and examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,

    /// The amount of disk space in GB to allocate on the local Pod volume.
    /// Pod volume provides persistent storage that survives instance restarts.
    /// Defaults to 20 GB if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,

    /// The absolute path where the Pod volume will be mounted in the container.
    /// This is where persistent storage will be accessible within the container.
    /// Defaults to "/workspace" if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Input parameters for updating existing templates.
///
/// Use this struct to modify template configuration. Template updates automatically
/// trigger rolling releases for any associated Serverless endpoints, ensuring
/// deployed instances use the latest configuration.
///
/// # Rolling Release Behavior
///
/// When a template is updated:
/// 1. New instances are created with the updated configuration
/// 2. Traffic is gradually shifted to new instances
/// 3. Old instances are gracefully terminated
/// 4. The process ensures zero-downtime updates for Serverless endpoints
///
/// # Partial Updates
///
/// All fields are optional, allowing for partial updates. Only specified fields
/// will be changed; unspecified fields retain their current values.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::TemplateUpdateInput;
///
/// // Update only the Docker image
/// let image_update = TemplateUpdateInput {
///     image_name: Some("myapp:v2.0.0".to_string()),
///     ..Default::default()
/// };
///
/// // Update storage allocation and environment variables
/// let config_update = TemplateUpdateInput {
///     container_disk_in_gb: Some(100),
///     volume_in_gb: Some(50),
///     env: Some({
///         let mut env = std::collections::HashMap::new();
///         env.insert("MODEL_VERSION".to_string(), "v2.0".to_string());
///         env.insert("BATCH_SIZE".to_string(), "32".to_string());
///         env
///     }),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateUpdateInput {
    /// Update the amount of disk space in GB for the container disk.
    /// Container disk provides fast local storage but data is wiped on restart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_disk_in_gb: Option<i32>,

    /// Update the container registry authentication ID.
    /// Use when changing to a private image that requires different credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_auth_id: Option<String>,

    /// Update the Docker image ENTRYPOINT override.
    /// Specify new entrypoint commands or set to empty to use image default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_entrypoint: Option<Vec<String>>,

    /// Update the Docker image start command override.
    /// Specify new start commands or set to empty to use image default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_start_cmd: Option<Vec<String>>,

    /// Update environment variables for the template.
    /// Completely replaces existing environment variables with the provided set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvVars>,

    /// Update the Docker image tag for the template.
    /// This is commonly updated to deploy new versions of applications.
    /// Triggers rolling release for associated Serverless endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,

    /// Update the template's public visibility.
    /// - `true`: Make template public (enables revenue sharing)
    /// - `false`: Make template private (restricts access to creator)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// Update the template name.
    /// New name must be unique within your account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Update the list of exposed ports.
    /// Each port is formatted as `[port]/[protocol]` (e.g., "8000/http").
    /// Completely replaces existing port configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,

    /// Update the template documentation.
    /// Provide new markdown-formatted description, usage instructions,
    /// and configuration details for the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,

    /// Update the amount of disk space in GB for the Pod volume.
    /// Pod volume provides persistent storage that survives instance restarts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,

    /// Update the Pod volume mount path.
    /// The absolute path where persistent storage will be accessible
    /// within containers deployed from this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Query parameters for listing templates with filtering options.
///
/// Control which templates are included in the response based on their
/// type, visibility, and binding status. All parameters are optional
/// and default to `false`, meaning only your private templates are returned by default.
///
/// # Filtering Strategy
///
/// Templates are categorized into different groups:
/// - **Private Templates**: Your own private templates (always included)
/// - **Public Templates**: Community-created public templates
/// - **RunPod Templates**: Official templates maintained by RunPod
/// - **Endpoint-bound Templates**: Templates currently used by Serverless endpoints
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::ListTemplatesQuery;
///
/// // Get all available templates (private + public + official)
/// let all_templates = ListTemplatesQuery {
///     include_public_templates: Some(true),
///     include_runpod_templates: Some(true),
///     include_endpoint_bound_templates: Some(true),
/// };
///
/// // Get only official RunPod templates
/// let official_only = ListTemplatesQuery {
///     include_runpod_templates: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplatesQuery {
    /// Include templates that are currently bound to Serverless endpoints.
    /// These templates are actively in use and may have restrictions on modification.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_endpoint_bound_templates: Option<bool>,

    /// Include community-made public templates in the response.
    /// Public templates can be used by anyone and may generate revenue for their creators.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public_templates: Option<bool>,

    /// Include official RunPod templates in the response.
    /// These are curated, maintained, and optimized templates provided by RunPod
    /// for popular frameworks and common use cases.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_runpod_templates: Option<bool>,
}

/// Query parameters for retrieving individual templates with filtering options.
///
/// Similar to [`ListTemplatesQuery`] but for single template retrieval.
/// Controls access to templates based on their type and visibility.
/// Useful when you need to access a specific template that might be public or official.
///
/// # Access Control
///
/// By default, only your own private templates are accessible. To access
/// public or official templates, you must explicitly enable the corresponding flags.
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::GetTemplateQuery;
///
/// // Access any type of template
/// let query = GetTemplateQuery {
///     include_public_templates: Some(true),
///     include_runpod_templates: Some(true),
///     include_endpoint_bound_templates: Some(true),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTemplateQuery {
    /// Include templates bound to Serverless endpoints in the search scope.
    /// Required if the target template is currently bound to an endpoint.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_endpoint_bound_templates: Option<bool>,

    /// Include public templates in the search scope.
    /// Required if the target template is a community-created public template.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public_templates: Option<bool>,

    /// Include official RunPod templates in the search scope.
    /// Required if the target template is an official RunPod template.
    /// Defaults to `false` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_runpod_templates: Option<bool>,
}
