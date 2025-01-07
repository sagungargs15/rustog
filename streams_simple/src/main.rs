

// This is a very basic example that shows:
// Creating a stream from an iterator
// Using async/await to process stream items
// Basic stream consumption pattern

use futures::stream::{self, StreamExt}; // StreamExt provides useful methods like next()
use async_std::task;  // For running async code

#[async_std::main]  // This attribute is needed for async main
async fn main() {
    // Create a simple stream that yields numbers 1,2,3
    let mut numbers = stream::iter(1..=3);
    
    // Consume the stream
    while let Some(number) = numbers.next().await {
        println!("Got number: {}", number);
    }
} 