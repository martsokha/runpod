//! Basic usage example demonstrating simple queries.
//! Run with: cargo run --example basic_usage

use runpod_sdk::model::v1::{ListEndpointsQuery, ListPodsQuery};
use runpod_sdk::service::v1::{EndpointsService, PodsService, TemplatesService};
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // List all pods
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

    // List endpoints
    println!("\nListing endpoints...");
    let endpoints = client.list_endpoints(ListEndpointsQuery::default()).await?;
    println!("Found {} endpoints", endpoints.len());

    for endpoint in &endpoints {
        println!(
            "  - {} ({})",
            endpoint.name.as_deref().unwrap_or("unnamed"),
            endpoint.id
        );
    }

    // List templates
    println!("\nListing templates...");
    let templates = client.list_templates(Default::default()).await?;
    println!("Found {} templates", templates.len());

    Ok(())
}
