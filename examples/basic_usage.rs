use runpod_sdk::model::ListPodsQuery;
use runpod_sdk::{Result, RunpodClient, RunpodConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = RunpodConfig::from_env()?;
    let client = RunpodClient::new(config)?;

    // List all pods
    println!("Listing pods...");
    let query = ListPodsQuery {
        include_machine: Some(true),
        ..Default::default()
    };
    let pods = client.pods().list(query).await?;
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
    let endpoints = client.endpoints().list(Default::default()).await?;
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
    let templates = client.templates().list(Default::default()).await?;
    println!("Found {} templates", templates.len());

    Ok(())
}
