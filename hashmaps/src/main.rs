use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.entry("Blue").or_insert(10);

    println!("{:?}", scores.get("Blue")); // Will Return Some(10)

    println!("{:?}", scores.get("Red")); // Will Return None Safely handling missing keys/values

    // Method 1: Pattern matching
    match scores.get("Blue") {
        Some(score) => println!("Score: {}", score),
        None => println!("Team not found"),
    }

    let score = scores.get("Red").unwrap_or(&0);
    println!("Score: {}", score);
}
