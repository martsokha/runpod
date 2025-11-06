# Runpod SDK

[![Crates.io](https://img.shields.io/crates/v/runpod-sdk.svg)](https://crates.io/crates/runpod-sdk)
[![Documentation](https://docs.rs/runpod-sdk/badge.svg)](https://docs.rs/runpod-sdk)
[![License](https://img.shields.io/crates/l/runpod-sdk.svg)](https://github.com/martsokha/runpod/blob/main/LICENSE.txt)
[![Build Status](https://github.com/martsokha/runpod/workflows/CI/badge.svg)](https://github.com/martsokha/runpod/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org)

A Rust client library for the [Runpod API](https://docs.runpod.io/). This SDK provides a type-safe, ergonomic interface for managing Pods, Serverless endpoints, templates, network volumes, and more.

## Features

- **Full API Coverage**: Support for all Runpod REST API endpoints
- **Type-Safe**: Strongly typed models with derive builder support
- **Async/Await**: Built on `reqwest` with async/await support
- **Well Documented**: Comprehensive documentation with examples
- **Easy Configuration**: Builder pattern for client configuration
- **Enum Conversions**: String conversions via `strum` for all enums
- **Optional Tracing**: Built-in tracing support via the `tracing` feature

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
runpod-sdk = "0.1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use runpod_sdk::{Config, RunpodClient};
use runpod_sdk::model::ListPodsQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API key
    let config = Config::builder()
        .api_key("your-api-key")
        .build()?;

    let client = RunpodClient::new(config)?;

    // List all pods
    let pods = client.pods().list(ListPodsQuery::default()).await?;
    println!("Found {} pods", pods.len());

    Ok(())
}
```

## Optional Features

### Tracing

Enable the `tracing` feature for built-in logging support:

```toml
[dependencies]
runpod-sdk = { version = "0.1.0", features = ["tracing"] }
```

This will add trace-level logs for HTTP requests and debug-level logs for client creation.

### Enum String Conversions

All enums support string conversions via `strum`:

```rust
use std::str::FromStr;
use runpod_sdk::model::{ComputeType, PodStatus};

// Convert from string
let compute_type = ComputeType::from_str("GPU")?;

// Convert to string
let status_str = PodStatus::Running.to_string(); // "RUNNING"
```

## Usage Examples

### Managing Pods

```rust
use runpod_sdk::model::{PodCreateInput, ComputeType};

// Create a new GPU pod
let input = PodCreateInput {
    name: Some("My GPU Pod".to_string()),
    image_name: Some("runpod/pytorch:latest".to_string()),
    gpu_count: Some(1),
    compute_type: Some(ComputeType::Gpu),
    ..Default::default()
};

let pod = client.pods().create(input).await?;
println!("Created pod: {}", pod.id);

// Get pod details
let pod = client.pods().get(&pod.id, Default::default()).await?;

// Start/stop a pod
client.pods().start(&pod.id).await?;
client.pods().stop(&pod.id).await?;

// Delete a pod
client.pods().delete(&pod.id).await?;
```

### Managing Serverless Endpoints

```rust
use runpod_sdk::model::{EndpointCreateInput, ScalerType};

// Create a serverless endpoint
let input = EndpointCreateInput {
    template_id: "your-template-id".to_string(),
    name: Some("My Endpoint".to_string()),
    workers_max: Some(5),
    workers_min: Some(0),
    scaler_type: Some(ScalerType::QueueDelay),
    scaler_value: Some(4),
    ..Default::default()
};

let endpoint = client.endpoints().create(input).await?;
println!("Created endpoint: {}", endpoint.id);

// List endpoints
let endpoints = client.endpoints().list(Default::default()).await?;
```

### Managing Templates

```rust
use runpod_sdk::model::TemplateCreateInput;

// Create a template
let input = TemplateCreateInput {
    name: "My Template".to_string(),
    image_name: "runpod/pytorch:latest".to_string(),
    is_serverless: Some(true),
    ..Default::default()
};

let template = client.templates().create(input).await?;
```

### Managing Network Volumes

```rust
use runpod_sdk::model::NetworkVolumeCreateInput;

// Create a network volume
let input = NetworkVolumeCreateInput {
    name: "My Volume".to_string(),
    size: 50,
    data_center_id: "EU-RO-1".to_string(),
};

let volume = client.volumes().create(input).await?;
```

### Billing Information

```rust
use runpod_sdk::model::{PodBillingQuery, BucketSize};

// Get pod billing history
let query = PodBillingQuery {
    bucket_size: Some(BucketSize::Day),
    start_time: Some("2023-01-01T00:00:00Z".to_string()),
    end_time: Some("2023-01-31T23:59:59Z".to_string()),
    ..Default::default()
};

let records = client.billing().pods(query).await?;
```

### Configuration with Builder

```rust
use runpod_sdk::Config;

let config = Config::builder()
    .api_key("your-api-key")
    .base_url("https://rest.runpod.io/v1")  // Optional, uses default if not set
    .timeout_secs(60)                        // Optional, default is 30
    .build()?;

let client = RunpodClient::new(config)?;
```

## API Services

The SDK provides the following services:

- **Pods**: Create, list, get, update, delete, start, stop, reset, and restart pods
- **Endpoints**: Create, list, get, update, and delete serverless endpoints
- **Templates**: Create, list, get, update, and delete templates
- **Network Volumes**: Create, list, get, update, and delete network volumes
- **Container Registry Auth**: Create, list, get, and delete container registry authentication
- **Billing**: Retrieve billing history for pods, endpoints, and network volumes

## Error Handling

The SDK uses a custom `Result` type with `RunpodError`:

```rust
use runpod_sdk::{Result, RunpodError};

match client.pods().list(Default::default()).await {
    Ok(pods) => println!("Found {} pods", pods.len()),
    Err(RunpodError::Http(e)) => eprintln!("HTTP error: {}", e),
    Err(RunpodError::Api(e)) => eprintln!("API error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

Check the `examples/` directory for more detailed usage examples:

```bash
# Set your API key
export RUNPOD_API_KEY="your-api-key"

# Run the basic usage example
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on how to submit pull requests, report issues, and contribute to the project.

## License

This project is licensed under the MIT License - see the [LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Runpod Documentation](https://docs.runpod.io/)
- [Runpod API Reference](https://rest.runpod.io/v1/docs)
