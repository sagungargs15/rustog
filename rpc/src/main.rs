//I'll help you create a Rust RPC call that's equivalent to the curl command.
// I'll use the reqwest crate which is commonly used for HTTP requests in Rust.

// 1. Uses async/await with tokio runtime
// 2. Includes basic authentication
// 3. Sets the proper Content-Type header
// 4. Sends the JSON-RPC request in the correct format
// 5. Handles the response as JSON

use anyhow::Result;
use serde_json::{json, Value}; // allow you to use the json! macro for creating JSON values.
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://boss2025.xyz:8332";
    let client = Client::new();
    let user = "user_324";
    let password = "Rn0tEynYp6tV";

    let response_001 = client
    .post(url)
    .basic_auth(user, Some(password))
    .header("Content-Type", "application/json")
    .json(&json!({
        "method": "getblockcount",
        "jsonrpc": "1.0",
        "id": "1"
    }))
    .send()
    .await?;

    let result_001: Value = response_001.json().await?;
    println!("Response_001: {:?}", serde_json::to_string_pretty(&result_001)?);


    // Step 1: Get the block hash for block 123456
    let response_002_blockhash = client
        .post(url)
        .basic_auth(user, Some(password))
        .header("Content-Type", "application/json")
        .json(&json!({
            "method": "getblockhash",
            "params": [123456],
            "jsonrpc": "1.0",
            "id": "1"
        }))
        .send()
        .await?;

    let result_002_blockhash: Value = response_002_blockhash.json().await?;
    println!("Response_002_blockhash: {:?}", serde_json::to_string_pretty(&result_002_blockhash)?);
    let block_hash = result_002_blockhash["result"].as_str().unwrap();

    // Step 2: Get the block data using the block hash
    let response_002_blockdata = client
        .post(url)
        .basic_auth(user, Some(password))
        .header("Content-Type", "application/json")
        .json(&json!({
            "method": "getblock",
            "params": [block_hash],
            "jsonrpc": "1.0",
            "id": "1"
        }))
        .send()
        .await?;

    let result_002_blockdata: Value = response_002_blockdata.json().await?;
    println!("Response_002_blockdata: {:?}", serde_json::to_string_pretty(&result_002_blockdata)?);    
    let transactions = result_002_blockdata["result"]["tx"].as_array().unwrap();
    println!("Transactions: {:?}", transactions);

    // Step 3: Count the outputs in each transaction
    let mut total_outputs = 0;
    for txid in transactions {
        let tx_data_response = client
            .post(url)
            .basic_auth(user, Some(password))
            .header("Content-Type", "application/json")
            .json(&json!({
                "method": "getrawtransaction",
                "params": [txid, true],
                "jsonrpc": "1.0",
                "id": "1"
            }))
            .send()
            .await?;

        let tx_data_result: Value = tx_data_response.json().await?;
        let outputs = tx_data_result["result"]["vout"].as_array().unwrap();
        total_outputs += outputs.len();
    }
    
    println!("Total new outputs in block 123,456: {}", total_outputs);

    Ok(())
}
