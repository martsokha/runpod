use runpod_sdk::model::ListEndpointsQuery;
use runpod_sdk::{Config, Result, RunpodClient};

// Uncomment the following if you want to use the commented-out creation example:
// use runpod_sdk::model::{EndpointCreateInput, GetEndpointQuery, ScalerType};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with your API key from environment
    let api_key =
        std::env::var("RUNPOD_API_KEY").expect("RUNPOD_API_KEY environment variable not set");

    let config = Config::builder().api_key(api_key).build()?;

    let client = RunpodClient::new(config)?;

    // List all endpoints with template and worker information
    println!("Listing all endpoints...");
    let query = ListEndpointsQuery {
        include_template: Some(true),
        include_workers: Some(true),
    };

    let endpoints = client.endpoints().list(query).await?;
    println!("Found {} endpoints\n", endpoints.len());

    for endpoint in &endpoints {
        println!(
            "Endpoint: {}",
            endpoint.name.as_deref().unwrap_or("unnamed")
        );
        println!("  ID: {}", endpoint.id);
        println!(
            "  Workers: {} min, {} max",
            endpoint.workers_min, endpoint.workers_max
        );
        println!("  Scaler Type: {:?}", endpoint.scaler_type);
        println!("  Version: {}", endpoint.version);

        if let Some(template) = &endpoint.template {
            println!("  Template: {}", template.name);
            println!("  Image: {}", template.image_name);
        }

        if let Some(workers) = &endpoint.workers {
            println!("  Active Workers: {}", workers.len());
        }
        println!();
    }

    Ok(())
}
