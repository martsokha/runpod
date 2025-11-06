use runpod_sdk::model::ListPodsQuery;
use runpod_sdk::{Config, Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with your API key
    let api_key =
        std::env::var("RUNPOD_API_KEY").expect("RUNPOD_API_KEY environment variable not set");

    let config = Config::builder().api_key(api_key).build()?;

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

    // Create a new pod (commented out to avoid accidental creation)
    // let input = PodCreateInput {
    //     name: Some("My Test Pod".to_string()),
    //     image_name: Some("runpod/pytorch:latest".to_string()),
    //     gpu_count: Some(1),
    //     ..Default::default()
    // };
    //
    // let pod = client.pods().create(input).await?;
    // println!("Created pod: {}", pod.id);

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
