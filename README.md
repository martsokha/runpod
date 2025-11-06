# Runpod SDK

[![Crates.io](https://img.shields.io/crates/v/runpod-sdk?style=flat-square&color=black)](https://crates.io/crates/runpod-sdk)
[![Documentation](https://img.shields.io/docsrs/runpod-sdk?style=flat-square&color=black)](https://docs.rs/runpod-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/martsokha/runpod/build.yml?style=flat-square&color=black)](https://github.com/martsokha/runpod/actions)

A Rust client library for the [Runpod API](https://docs.runpod.io/). This SDK provides a type-safe, ergonomic interface for managing Pods, Serverless endpoints, templates, network volumes, and more.

## Features

- **Full API Coverage**: Support for all Runpod REST API endpoints
- **Type-Safe**: Strongly typed models with derive builder support
- **Async/Await**: Built on `reqwest` with async/await support
- **Well Documented**: Comprehensive documentation with examples
- **Easy Configuration**: Builder pattern for client configuration
- **Optional Features**: Modular features for enum conversions and tracing
- **Secure**: API keys are automatically masked in debug output

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
runpod-sdk = "0.1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use runpod_sdk::{RunpodClient, RunpodConfig};
use runpod_sdk::model::ListPodsQuery;

#[tokio::main]
async fn main() -> runpod_sdk::Result<()> {
    // Create a client with your API key
    let client = RunpodConfig::builder()
        .with_api_key("your-api-key")
        .build_client()?;

    // List all pods
    let pods = client.pods().list(ListPodsQuery::default()).await?;
    println!("Found {} pods", pods.len());

    Ok(())
}
```

## Optional Features

The SDK includes several optional features that can be enabled as needed:

### Tracing

Enable comprehensive logging and tracing support:

```toml
[dependencies]
runpod-sdk = { version = "0.1.0", features = ["tracing"] }
```

This adds trace-level logs for HTTP requests and debug-level logs for client operations.

### Enum String Conversions

Enable string conversion methods for all enums:

```toml
[dependencies]
runpod-sdk = { version = "0.1.0", features = ["strum"] }
```

When enabled, all enums support parsing from and converting to strings:

```rust
#[cfg(feature = "strum")]
use std::str::FromStr;
#[cfg(feature = "strum")]  
use runpod_sdk::model::{ComputeType, PodStatus};

#[cfg(feature = "strum")]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Convert from string
    let compute_type = ComputeType::from_str("GPU")?;

    // Convert to string  
    let status_str = PodStatus::Running.to_string(); // "RUNNING"
    Ok(())
}
```

### All Features

Enable all optional features:

```toml
[dependencies]
runpod-sdk = { version = "0.1.0", features = ["tracing", "strum"] }
```

## Configuration Options

The SDK offers flexible configuration with multiple approaches:

### Quick Start with Builder

```rust
use runpod_sdk::RunpodConfig;

fn example() -> runpod_sdk::Result<()> {
    let client = RunpodConfig::builder()
        .with_api_key("your-api-key")
        .build_client()?;
    
    Ok(())
}
```

### Environment Variables

```rust
use runpod_sdk::RunpodConfig;

fn example() -> runpod_sdk::Result<()> {
    // Uses RUNPOD_API_KEY, RUNPOD_BASE_URL, RUNPOD_TIMEOUT_SECS
    let client = RunpodConfig::from_env()?.build_client()?;
    Ok(())
}
```

### Advanced Configuration

```rust
use runpod_sdk::RunpodConfig;
use std::time::Duration;

fn example() -> runpod_sdk::Result<()> {
    let client = RunpodConfig::builder()
        .with_api_key("your-api-key")
        .with_base_url("https://api.runpod.io/v1")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;
    Ok(())
}
```

## Additional Features

The SDK provides comprehensive support for all RunPod API operations including:

- **Pods**: Complete lifecycle management (create, list, get, update, delete, start, stop, reset, restart)
- **Serverless Endpoints**: Full endpoint management and scaling configuration
- **Templates**: Template creation and management for reusable configurations  
- **Network Volumes**: Storage volume provisioning and management
- **Container Registry**: Authentication and registry management
- **Billing**: Detailed usage and billing information retrieval

For complete API documentation and examples, see the [full documentation on docs.rs](https://docs.rs/runpod-sdk).

## Examples

The `examples/` directory contains comprehensive usage examples. To run them:

```bash
# Set your API key
export RUNPOD_API_KEY="your-api-key"

# Run the basic usage example
cargo run --example basic_usage

# Run the endpoints management example
cargo run --example manage_endpoints
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on how to submit pull requests, report issues, and contribute to the project.

## License

This project is licensed under the MIT License - see the [LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Runpod Documentation](https://docs.runpod.io/)
- [Runpod API Reference](https://rest.runpod.io/v1/docs)
