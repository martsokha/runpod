//! Comprehensive pod lifecycle management example.
//!
//! This example demonstrates:
//! - Listing existing pods with machine details
//! - Creating a new pod with custom configuration
//! - Monitoring pod status during startup
//! - Updating pod properties
//! - Controlling pod state (stop/start/reset)
//! - Filtering pods by status
//!
//! Run with: cargo run --example manage_pods

use std::time::Duration;

use runpod_sdk::model::v1::{
    CloudType, GetPodQuery, GpuTypeId, ListPodsQuery, PodCreateInput, PodStatus, PodUpdateInput,
};
use runpod_sdk::service::v1::PodsService;
use runpod_sdk::{Result, RunpodClient};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Create client from RUNPOD_API_KEY environment variable
    let client = RunpodClient::from_env()?;

    // List existing pods
    println!("Listing existing pods...");
    let query = ListPodsQuery {
        include_machine: Some(true),
        ..Default::default()
    };
    let pods = client.list_pods(query).await?;
    println!("Found {} pods", pods.len());

    for pod in &pods {
        println!(
            "  - {} ({}): {:?}",
            pod.name.as_deref().unwrap_or("unnamed"),
            pod.id,
            pod.desired_status
        );
    }

    // Create a new pod
    println!("\nCreating new pod...");
    let create_input = PodCreateInput {
        name: Some("test-pod".to_string()),
        image_name: Some("runpod/pytorch:2.0.1-py3.10-cuda11.8.0-devel-ubuntu22.04".to_string()),
        gpu_type_ids: Some(vec![GpuTypeId::NvidiaGeForceRtx3070]),
        cloud_type: Some(CloudType::Secure),
        volume_in_gb: Some(10),
        container_disk_in_gb: Some(5),
        env: Some({
            let mut env = std::collections::HashMap::new();
            env.insert("TEST_VAR".to_string(), "test_value".to_string());
            env
        }),
        ..Default::default()
    };

    match client.create_pod(create_input).await {
        Ok(pod) => {
            println!("Created pod: {}", pod.id);
            let pod_id = pod.id.clone();

            // Monitor pod status
            println!("Monitoring pod status...");
            for i in 1..=5 {
                sleep(Duration::from_secs(10)).await;

                let query = GetPodQuery {
                    include_machine: Some(true),
                    ..Default::default()
                };

                if let Ok(pod_info) = client.get_pod(&pod_id, query).await {
                    println!("  Check {}: {:?}", i, pod_info.desired_status);

                    if matches!(
                        pod_info.desired_status,
                        PodStatus::Running | PodStatus::Exited | PodStatus::Terminated
                    ) {
                        break;
                    }
                }
            }

            // Update pod
            println!("Updating pod...");
            let update_input = PodUpdateInput {
                name: Some("updated-test-pod".to_string()),
                ..Default::default()
            };

            if let Err(e) = client.update_pod(&pod_id, update_input).await {
                println!("Update failed: {}", e);
            } else {
                println!("Pod updated successfully");
            }

            // Stop pod
            println!("Stopping pod...");
            if let Err(e) = client.stop_pod(&pod_id).await {
                println!("Stop failed: {}", e);
            } else {
                println!("Pod stopped");
            }

            // Note: Uncomment to delete the pod
            // client.delete_pod(&pod_id).await?;
            println!("Pod cleanup skipped (uncomment delete line if needed)");
        }
        Err(e) => {
            println!("Failed to create pod: {}", e);
        }
    }

    // List running pods only
    println!("\nListing running pods...");
    let running_query = ListPodsQuery {
        desired_status: Some(PodStatus::Running),
        include_machine: Some(true),
        ..Default::default()
    };

    let running_pods = client.list_pods(running_query).await?;
    println!("Running pods: {}", running_pods.len());

    for pod in running_pods {
        if let Some(machine) = &pod.machine {
            println!(
                "  - {} on {} (${}/hr)",
                pod.name.as_deref().unwrap_or("unnamed"),
                machine.gpu_display_name.as_deref().unwrap_or("unknown"),
                machine.cost_per_hr
            );
        }
    }

    Ok(())
}
