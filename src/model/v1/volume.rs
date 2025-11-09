use serde::{Deserialize, Serialize};

pub use super::common::NetworkVolume;

/// List of network volumes.
///
/// A collection type representing multiple network volumes, typically returned
/// from API endpoints that list volumes for an account or data center.
pub type NetworkVolumes = Vec<NetworkVolume>;

/// Input parameters for creating a new network volume.
///
/// This struct contains all the required configuration options for creating a network volume.
/// All fields are mandatory as they define the fundamental characteristics of the volume
/// that cannot be changed after creation (except size, which can only be increased).
///
/// # Validation Requirements
///
/// - **name**: Must be 1-255 characters long. Can contain letters, numbers, spaces, hyphens, and underscores
/// - **size**: Must be between 1 and 4,000 GB. Choose based on your storage needs and budget
/// - **data_center_id**: Must be a valid RunPod data center identifier (format: XX-XX-N)
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::v1::NetworkVolumeCreateInput;
///
/// // Create a small development volume
/// let dev_volume = NetworkVolumeCreateInput {
///     name: "dev-workspace".to_string(),
///     size: 10,
///     data_center_id: "US-CA-1".to_string(),
/// };
///
/// // Create a large production dataset volume
/// let prod_volume = NetworkVolumeCreateInput {
///     name: "production-ml-datasets".to_string(),
///     size: 1000,
///     data_center_id: "EU-RO-1".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeCreateInput {
    /// A user-defined name for the network volume.
    ///
    /// The name is used for identification and organization purposes.
    /// It does not need to be unique across your account, allowing you to
    /// use descriptive names that match your workflow or project structure.
    ///
    /// **Constraints:**
    /// - Length: 1-255 characters
    /// - Allowed characters: letters, numbers, spaces, hyphens, underscores
    /// - Case-sensitive
    ///
    /// **Examples:** "ml-training-data", "user uploads", "backup_volume_2024"
    pub name: String,

    /// The amount of disk space, in gigabytes (GB), to allocate to the network volume.
    ///
    /// This determines the storage capacity of the volume and directly affects billing.
    /// Choose a size that accounts for current needs plus reasonable growth, as expanding
    /// volumes requires an update operation and may take time to complete.
    ///
    /// **Constraints:**
    /// - Minimum: 1 GB
    /// - Maximum: 4,000 GB (4 TB)
    /// - Billing: Charged per GB-hour for the full allocated capacity
    ///
    /// **Performance notes:**
    /// - Larger volumes may have better IOPS performance
    /// - Size can be increased later but never decreased
    ///
    /// **Examples:** 10 (small dev), 100 (medium project), 1000 (large dataset)
    pub size: u32,

    /// The RunPod data center ID where the network volume will be created.
    ///
    /// Network volumes are bound to specific data centers and can only be attached
    /// to Pods running in the same data center. Choose based on:
    /// - Geographic proximity to your users
    /// - Data sovereignty requirements
    /// - Pricing differences between regions
    /// - Availability of required GPU/CPU types
    ///
    /// **Format:** Two-letter country code, two-letter region code, and number (XX-XX-N)
    ///
    /// **Common data centers:**
    /// - `US-CA-1`: California, USA (West Coast)
    /// - `US-TX-1`: Texas, USA (Central)
    /// - `EU-RO-1`: Romania, Europe
    /// - `EU-SE-1`: Sweden, Europe
    ///
    /// **Note:** Available data centers and their identifiers can be retrieved
    /// from the data centers API endpoint.
    pub data_center_id: String,
}

/// Input parameters for updating an existing network volume.
///
/// This struct allows you to modify the name and/or size of an existing network volume.
/// Both fields are optional, allowing you to update only the properties you want to change.
///
/// # Important Notes
///
/// - **Size expansion only**: You can increase the volume size but never decrease it
/// - **Live expansion**: Size changes can be performed while Pods are using the volume
/// - **Billing impact**: Size increases affect billing immediately
/// - **No downtime**: Name changes are instantaneous with no service interruption
///
/// # Validation Requirements
///
/// - **name**: If provided, must be 1-255 characters long
/// - **size**: If provided, must be larger than current size and ≤4,000 GB
///
/// # Examples
///
/// ```rust
/// use runpod_sdk::model::v1::NetworkVolumeUpdateInput;
///
/// // Only change the name
/// let rename_only = NetworkVolumeUpdateInput {
///     name: Some("renamed-volume".to_string()),
///     size: None,
/// };
///
/// // Only expand the size from current to 500GB
/// let expand_only = NetworkVolumeUpdateInput {
///     name: None,
///     size: Some(500),
/// };
///
/// // Change both name and size
/// let full_update = NetworkVolumeUpdateInput {
///     name: Some("production-storage-v2".to_string()),
///     size: Some(1000),
/// };
///
/// // No changes (useful for testing API connectivity)
/// let no_change = NetworkVolumeUpdateInput::default();
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVolumeUpdateInput {
    /// Optional new name for the network volume.
    ///
    /// If provided, the volume will be renamed to this value. The name change
    /// is applied immediately and does not affect volume availability or performance.
    ///
    /// **Constraints:**
    /// - Length: 1-255 characters (if provided)
    /// - Allowed characters: letters, numbers, spaces, hyphens, underscores
    /// - Case-sensitive
    ///
    /// **Use cases:**
    /// - Updating naming conventions across your infrastructure
    /// - Adding version numbers or status indicators
    /// - Improving organization and searchability
    ///
    /// **Note:** Set to `None` to keep the current name unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional new size for the network volume in gigabytes (GB).
    ///
    /// If provided, the volume will be expanded to this new size. The expansion
    /// operation preserves all existing data and can be performed while Pods are
    /// actively using the volume.
    ///
    /// **Constraints:**
    /// - Must be larger than the current volume size (expansion only)
    /// - Maximum: 4,000 GB (4 TB)
    /// - Minimum increment: 1 GB
    ///
    /// **Process:**
    /// 1. API call returns immediately with success
    /// 2. Volume expansion happens asynchronously in the background
    /// 3. Additional capacity becomes available once expansion completes
    /// 4. Billing for the new size begins immediately
    ///
    /// **Performance impact:**
    /// - No downtime during expansion
    /// - File system may need manual extension in some cases
    /// - Larger volumes may have improved IOPS performance
    ///
    /// **Note:** Set to `None` to keep the current size unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}
