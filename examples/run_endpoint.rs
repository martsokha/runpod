//! Example demonstrating how to run jobs on RunPod serverless endpoints.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! export RUNPOD_ENDPOINT_ID="your-endpoint-id"
//! cargo run --example run_endpoint --features serverless
//! ```

use runpod_sdk::serverless::Endpoint;
use runpod_sdk::{Result, RunpodClient};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint_id = std::env::var("RUNPOD_ENDPOINT_ID")
        .expect("RUNPOD_ENDPOINT_ID environment variable not set");

    println!("RunPod Endpoint Runner Example\n");

    let client = RunpodClient::from_env()?;
    println!("Client created successfully\n");

    let endpoint = Endpoint::new(endpoint_id, client);
    println!(
        "Endpoint instance created for: {}\n",
        endpoint.endpoint_id()
    );

    // Example 1: Check endpoint health
    println!("Example 1: Checking endpoint health...");
    match endpoint.health().await {
        Ok(health) => {
            println!("  Jobs:");
            println!("    - Completed: {}", health.jobs.completed);
            println!("    - Failed: {}", health.jobs.failed);
            println!("    - In Progress: {}", health.jobs.in_progress);
            println!("    - In Queue: {}", health.jobs.in_queue);
            println!("  Workers:");
            println!("    - Ready: {}", health.workers.ready);
            println!("    - Running: {}", health.workers.running);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Example 2: Run job
    println!("Example 2: Running job...");
    let input = json!({"prompt": "Hello, World!"});

    match endpoint.run(&input) {
        Ok(job) => {
            println!("  Job created");

            match job.await {
                Ok(output) => println!("  Job output: {:?}", output),
                Err(e) => println!("  Error running job: {}", e),
            }
        }
        Err(e) => println!("  Error creating job: {}", e),
    }
    println!();

    // Example 3: Stream job results
    println!("Example 3: Streaming job results...");
    let input = json!({"prompt": "Generate streaming output"});

    match endpoint.run(&input) {
        Ok(job) => {
            println!("  Streaming job created");

            let mut iteration = 0;
            loop {
                match job.stream().await {
                    Ok((status, chunks)) => {
                        if !chunks.is_empty() {
                            println!("  Received {} chunk(s):", chunks.len());
                            for chunk in chunks {
                                println!("    - {:?}", chunk.output);
                            }
                        }

                        if status.is_final() {
                            println!("  Stream completed with status: {:?}", status);
                            break;
                        }

                        iteration += 1;
                        if iteration > 30 {
                            println!("  Stream timeout after 30 iterations");
                            break;
                        }

                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    Err(e) => {
                        println!("  Error streaming: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("  Error creating streaming job: {}", e),
    }
    println!();

    // Example 4: Cancel a job
    println!("Example 4: Cancelling a job...");
    let input = json!({"prompt": "This job will be cancelled"});

    match endpoint.run(&input) {
        Ok(job) => {
            println!("  Job created");

            std::thread::sleep(std::time::Duration::from_secs(1));

            match job.cancel().await {
                Ok(_) => println!("  Job cancelled successfully"),
                Err(e) => println!("  Error cancelling job: {}", e),
            }

            match job.status().await {
                Ok(status) => println!("  Status after cancel: {:?}", status),
                Err(e) => println!("  Error getting status: {}", e),
            }
        }
        Err(e) => println!("  Error creating job: {}", e),
    }
    println!();

    println!("All examples completed!");

    Ok(())
}
