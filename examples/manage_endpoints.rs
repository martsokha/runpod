use runpod_sdk::model::ListEndpointsQuery;
use runpod_sdk::service::EndpointsService;
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // List all endpoints with template and worker information
    println!("Listing all endpoints...");
    let query = ListEndpointsQuery {
        include_template: Some(true),
        include_workers: Some(true),
    };

    let endpoints = client.list_endpoints(query).await?;
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
