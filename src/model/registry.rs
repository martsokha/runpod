//! Container registry authentication models for the RunPod API.
//!
//! This module contains all the data structures and types needed to manage container registry
//! authentication credentials in RunPod. These credentials allow RunPod to pull private Docker
//! images from protected container registries during Pod and Serverless endpoint deployment.
//!
//! # Overview
//!
//! RunPod's container registry authentication system enables secure access to private Docker images:
//!
//! - **Private Registry Support**: Authenticate with Docker Hub, AWS ECR, Google Container Registry, and other registries
//! - **Secure Credential Storage**: Safely store registry credentials without exposing sensitive information
//! - **Reusable Authentication**: Create named credentials that can be referenced across multiple deployments
//! - **Pod Integration**: Seamlessly use private images in Pod and Serverless endpoint configurations
//! - **Access Control**: Manage which images and registries your RunPod resources can access
//!
//! # Core Types
//!
//! - [`ContainerRegistryAuth`]: Represents stored container registry authentication credentials
//! - [`ContainerRegistryAuthCreateInput`]: Input parameters for creating new registry authentication
//! - [`ContainerRegistryAuths`]: Collection of container registry authentication records
//!
//! # Authentication Workflow
//!
//! 1. **Create Authentication**: Use [`ContainerRegistryAuthCreateInput`] to store registry credentials
//! 2. **Reference in Deployments**: Use the authentication ID when creating Pods or endpoints with private images
//! 3. **Automatic Access**: RunPod automatically uses the credentials to pull private images during deployment
//! 4. **Management**: List, retrieve, and delete authentication records as needed
//!
//! # Supported Registries
//!
//! The authentication system works with any Docker-compatible registry:
//!
//! - **Docker Hub**: For private Docker Hub repositories
//! - **AWS ECR**: Amazon Elastic Container Registry
//! - **Google Container Registry (GCR)**: Google Cloud's container registry
//! - **Azure Container Registry (ACR)**: Microsoft Azure's container registry
//! - **GitHub Container Registry (GHCR)**: GitHub's container registry
//! - **Self-hosted Registries**: Private Docker registries and Harbor installations
//!
//! # Security Considerations
//!
//! - Credentials are securely stored and encrypted by RunPod
//! - Passwords and tokens are never returned in API responses
//! - Use registry-specific access tokens instead of account passwords when possible
//! - Regularly rotate authentication credentials for enhanced security
//!
//! # Examples
//!
//! ```rust
//! use runpod_sdk::model::registry::ContainerRegistryAuthCreateInput;
//!
//! // Create authentication for a private Docker Hub repository
//! let auth_input = ContainerRegistryAuthCreateInput {
//!     name: "my-dockerhub-auth".to_string(),
//!     username: "myusername".to_string(),
//!     password: "my-access-token".to_string(),
//! };
//!
//! // Authentication records are obtained from API responses
//! // and contain only non-sensitive information
//! ```

use serde::{Deserialize, Serialize};

/// Container registry authentication credentials.
///
/// Represents stored authentication credentials for accessing private container registries.
/// These credentials enable RunPod to pull private Docker images during Pod and Serverless
/// endpoint deployment.
///
/// # Security Notice
///
/// This struct only contains non-sensitive information. Actual passwords and tokens
/// are securely stored by RunPod and never returned in API responses.
///
/// # Usage
///
/// Authentication records are typically referenced by their ID when creating Pods or
/// Serverless endpoints that require access to private container images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRegistryAuth {
    /// A unique string identifying the container registry authentication.
    /// This ID is used to reference the authentication when creating Pods or endpoints.
    pub id: String,

    /// A user-defined name for the container registry authentication.
    /// The name must be unique within your RunPod account and should be descriptive
    /// enough to identify the registry or purpose (e.g., "production-ecr", "my-dockerhub").
    pub name: String,
}

/// Collection of container registry authentication records.
///
/// This type alias represents the standard response format when listing
/// container registry authentications, containing an array of [`ContainerRegistryAuth`] instances.
pub type ContainerRegistryAuths = Vec<ContainerRegistryAuth>;

/// Input parameters for creating new container registry authentication.
///
/// Use this struct to provide the necessary credentials for accessing a private container registry.
/// All fields are required to establish authentication with the registry.
///
/// # Security Best Practices
///
/// - Use registry-specific access tokens instead of account passwords when available
/// - Ensure the username and password/token combination has the minimum required permissions
/// - Use descriptive names that help identify the registry and intended use case
/// - Regularly rotate credentials to maintain security
///
/// # Registry-Specific Examples
///
/// ## Docker Hub
/// ```rust
/// # use runpod_sdk::model::registry::ContainerRegistryAuthCreateInput;
/// let docker_hub_auth = ContainerRegistryAuthCreateInput {
///     name: "dockerhub-production".to_string(),
///     username: "myusername".to_string(),
///     password: "dckr_pat_1234567890abcdef".to_string(), // Docker access token
/// };
/// ```
///
/// ## AWS ECR
/// ```rust
/// # use runpod_sdk::model::registry::ContainerRegistryAuthCreateInput;
/// let ecr_auth = ContainerRegistryAuthCreateInput {
///     name: "aws-ecr-us-west-2".to_string(),
///     username: "AWS".to_string(),
///     password: "eyJwYXlsb2FkIjoi...".to_string(), // ECR authorization token
/// };
/// ```
///
/// ## GitHub Container Registry
/// ```rust
/// # use runpod_sdk::model::registry::ContainerRegistryAuthCreateInput;
/// let ghcr_auth = ContainerRegistryAuthCreateInput {
///     name: "github-packages".to_string(),
///     username: "myusername".to_string(),
///     password: "ghp_1234567890abcdef".to_string(), // GitHub personal access token
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRegistryAuthCreateInput {
    /// A user-defined name for the container registry authentication.
    /// The name must be unique within your RunPod account. Choose a descriptive
    /// name that identifies the registry and its intended use case.
    ///
    /// Examples: "production-ecr", "staging-dockerhub", "private-harbor"
    pub name: String,

    /// The username for authenticating with the container registry.
    /// This varies by registry type:
    /// - Docker Hub: Your Docker Hub username
    /// - AWS ECR: Always "AWS"
    /// - GitHub Container Registry: Your GitHub username
    /// - Google Container Registry: "_json_key" for service account authentication
    pub username: String,

    /// The password, token, or credential for authenticating with the container registry.
    /// For enhanced security, use registry-specific access tokens when available:
    /// - Docker Hub: Use access tokens instead of passwords
    /// - AWS ECR: Use ECR authorization tokens
    /// - GitHub: Use personal access tokens with package permissions
    /// - Google GCR: Use service account JSON key (as string) for the password
    pub password: String,
}
