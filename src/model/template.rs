use serde::{Deserialize, Serialize};

use super::common::*;

/// Template category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    #[serde(rename = "NVIDIA")]
    Nvidia,
    #[serde(rename = "AMD")]
    Amd,
    #[serde(rename = "CPU")]
    Cpu,
}

/// Template resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub image_name: String,
    pub is_public: bool,
    pub is_runpod: bool,
    pub is_serverless: bool,
    pub category: Option<TemplateCategory>,
    pub container_disk_in_gb: i32,
    pub container_registry_auth_id: Option<String>,
    pub docker_entrypoint: Option<Vec<String>>,
    pub docker_start_cmd: Option<Vec<String>>,
    pub earned: Option<f64>,
    pub env: Option<EnvVars>,
    pub ports: Option<Vec<String>>,
    pub readme: Option<String>,
    pub runtime_in_min: Option<i32>,
    pub volume_in_gb: i32,
    pub volume_mount_path: String,
}

/// List of templates
pub type Templates = Vec<Template>;

/// Input for creating a template
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateCreateInput {
    pub name: String,
    pub image_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TemplateCategory>,
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
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_serverless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Input for updating a template (triggers rolling release)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateUpdateInput {
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
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_in_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mount_path: Option<String>,
}

/// Query parameters for listing templates
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplatesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_endpoint_bound_templates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public_templates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_runpod_templates: Option<bool>,
}

/// Query parameters for getting a single template
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTemplateQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_endpoint_bound_templates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public_templates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_runpod_templates: Option<bool>,
}
