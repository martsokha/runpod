//! Pod lifecycle management example.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! cargo run --example manage_pods
//! ```

use runpod_sdk::model::{CloudType, ListPodsQuery, PodCreateInput, PodUpdateInput};
use runpod_sdk::service::PodsService;
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // List existing pods
    println!("Listing pods...");
    let query = ListPodsQuery {
        include_machine: Some(true),
        ..Default::default()
    };
    let pods = client.list_pods(query).await?;
    println!("Found {} pods", pods.len());

    for pod in &pods {
        println!(
            "  - {} ({})",
            pod.name.as_deref().unwrap_or("unnamed"),
            pod.id
        );
    }

    // Create a new pod (with minimal required fields)
    println!("\nCreating pod...");
    let create_input = PodCreateInput {
        name: Some("example-pod".to_string()),
        image_name: Some("runpod/pytorch:2.0.1-py3.10-cuda11.8.0-devel-ubuntu22.04".to_string()),
        cloud_type: Some(CloudType::Community),
        volume_in_gb: Some(10),
        ..Default::default()
    };

    let pod = client.create_pod(create_input).await?;
    println!("Created pod: {}", pod.id);
    let pod_id = pod.id;

    // Update pod
    println!("\nUpdating pod...");
    let update_input = PodUpdateInput {
        name: Some("updated-example-pod".to_string()),
        ..Default::default()
    };
    client.update_pod(&pod_id, update_input).await?;
    println!("Pod updated");

    // Stop pod
    println!("\nStopping pod...");
    client.stop_pod(&pod_id).await?;
    println!("Pod stopped");

    // Cleanup
    client.delete_pod(&pod_id).await?;

    Ok(())
}
