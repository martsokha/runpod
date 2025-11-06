use serde::{Deserialize, Serialize};

pub use super::common::NetworkVolume;

/// List of network volumes.
pub type NetworkVolumes = Vec<NetworkVolume>;

/// Input for creating a network volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeCreateInput {
    pub name: String,
    pub size: i32,
    pub data_center_id: String,
}

/// Input for updating a network volume.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
