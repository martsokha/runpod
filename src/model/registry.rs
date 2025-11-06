use serde::{Deserialize, Serialize};

/// Container registry authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRegistryAuth {
    pub id: String,
    pub name: String,
}

/// List of container registry auths.
pub type ContainerRegistryAuths = Vec<ContainerRegistryAuth>;

/// Input for creating a container registry auth.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRegistryAuthCreateInput {
    pub name: String,
    pub username: String,
    pub password: String,
}
