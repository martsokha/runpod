//! Endpoint management example.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! cargo run --example manage_endpoints
//! ```

use runpod_sdk::model::ListEndpointsQuery;
use runpod_sdk::service::EndpointsService;
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // List endpoints
    println!("Listing endpoints...");
    let query = ListEndpointsQuery {
        include_template: Some(true),
        include_workers: Some(true),
    };
    let endpoints = client.list_endpoints(query).await?;
    println!("Found {} endpoints", endpoints.len());

    for endpoint in &endpoints {
        println!(
            "  - {} ({})",
            endpoint.name.as_deref().unwrap_or("unnamed"),
            endpoint.id
        );
    }

    // Display first endpoint details
    if let Some(endpoint) = endpoints.first() {
        println!(
            "\nEndpoint Details: {}",
            endpoint.name.as_deref().unwrap_or("unnamed")
        );
        println!(
            "  Workers: {} min, {} max",
            endpoint.workers_min, endpoint.workers_max
        );
        println!("  Scaler: {:?}", endpoint.scaler_type);

        if let Some(template) = &endpoint.template {
            println!("  Template: {}", template.name);
            println!("  Image: {}", template.image_name);
        }

        if let Some(workers) = &endpoint.workers {
            println!("  Active Workers: {}", workers.len());
        }
    }

    Ok(())
}
