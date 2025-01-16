//I'll help you create a Rust RPC call that's equivalent to the curl command.
// I'll use the reqwest crate which is commonly used for HTTP requests in Rust.

// 1. Uses async/await with tokio runtime
// 2. Includes basic authentication
// 3. Sets the proper Content-Type header
// 4. Sends the JSON-RPC request in the correct format
// 5. Handles the response as JSON

use anyhow::Result;
use serde_json::{json, Value}; // allow you to use the json! macro for creating JSON values.

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://boss2025.xyz:8332";
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .basic_auth("user_324", Some("Rn0tEynYp6tV"))
        .header("Content-Type", "application/json")
        .json(&json!({
            "method": "getblockcount",
            "jsonrpc": "1.0",
            "id": "1"
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    println!("Response: {:?}", result);

    Ok(())
}
