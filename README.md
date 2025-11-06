# Runpod SDK

This crate is a fork of the original [runpod.rs](https://github.com/agentsea/runpod.rs) developed by [Patrick Barker](https://github.com/pbarker).

[![Crates.io](https://img.shields.io/crates/v/runpod-sdk?style=flat-square&color=black)](https://crates.io/crates/runpod-sdk)
[![Documentation](https://img.shields.io/docsrs/runpod-sdk?style=flat-square&color=black)](https://docs.rs/runpod-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/martsokha/runpod/build.yml?style=flat-square&color=black)](https://github.com/martsokha/runpod/actions)

A Rust client library for the [Runpod API](https://docs.runpod.io/). This SDK provides a type-safe, ergonomic interface for managing Pods, Serverless endpoints, templates, network volumes, and more.

## Features

- **Pod Management**: Complete lifecycle operations (create, list, update, delete, start, stop, etc.)
- **Serverless Endpoints**: Full endpoint management with auto-scaling configuration
- **Template Management**: Reusable pod templates with custom configurations
- **Network Volumes**: Persistent storage volumes across pods
- **Type Safety**: Strongly typed models with comprehensive validation
- **Async/Await**: Built on modern async Rust with `tokio` and `reqwest`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
runpod-sdk = { version = "0.1", features = [] }
```

## Quick Start

### Builder Configuration

```rust,no_run
use runpod_sdk::{RunpodConfig, Result};
use runpod_sdk::model::ListPodsQuery;
use runpod_sdk::service::PodsService;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodConfig::builder()
        .with_api_key("your-api-key")
        .with_base_url("https://api.runpod.io/v1")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;

    let pods = client.list_pods(ListPodsQuery::default()).await?;
    println!("Found {} pods", pods.len());

    Ok(())
}
```

### Environment Variables

```rust,no_run
use runpod_sdk::{RunpodClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Uses RUNPOD_API_KEY, RUNPOD_BASE_URL, RUNPOD_BASE_GRAPHQL_URL (with graphql feature), RUNPOD_TIMEOUT_SECS
    let client = RunpodClient::from_env()?;
    Ok(())
}
```

## Optional Features

### TLS Backend

Choose between two TLS implementations:

```toml
# Default: rustls-tls (recommended)
runpod-sdk = { version = "0.1.0", features = [] }

# Alternative: native-tls
runpod-sdk = { version = "0.1.0", features = ["native-tls"], default-features = false }
```

### Tracing Support

Enable comprehensive logging and tracing:

```toml
runpod-sdk = { version = "0.1.0", features = ["tracing"] }
```

### Enum String Conversions

Enable string parsing and conversion for all enums:

```toml
runpod-sdk = { version = "0.1.0", features = ["strum"] }
```

## Examples

The `examples/` directory contains comprehensive usage examples:

```bash
# Set your API key
export RUNPOD_API_KEY="your-api-key"

# Run the basic usage example
cargo run --example basic_usage

# Run the endpoints management example
cargo run --example manage_endpoints

# Run the pods management example
cargo run --example manage_pods
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on how to submit pull requests, report issues, and contribute to the project.

## License

This project is licensed under the MIT License - see the [LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Runpod Documentation](https://docs.runpod.io/)
- [Runpod API Reference](https://rest.runpod.io/v1/docs)
- [Full API Documentation](https://docs.rs/runpod-sdk)
