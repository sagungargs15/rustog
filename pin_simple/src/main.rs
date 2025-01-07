


// Basic Usage: The Pin wrapper ensures our Timer stays in one place in memory
// Safety: Even though this example is simple, Pin is required by the Future trait
// Real World: In real async code, futures often contain references to their own data

// This shows the basic mechanics of Pin without getting into the complexities of self-referential structs. 
// The key point is that Pin ensures data stays at a fixed memory location, which is crucial for async programming 
// and certain data structures.

//You actually don't need to add anything to Cargo.toml because Pin is part of the standard library (std::pin::Pin).
use std::pin::Pin;

// Regular struct
struct MyData {
    value: i32,
}

fn main() {
    // Three ways to create pinned data:

    // 1. Pin to the heap (most common)
    let heap_pinned = Box::pin(MyData { value: 42 });

    // 2. Pin in place
    use std::pin::pin;
    let mut _stack_pinned = pin!(MyData { value: 42 });

    // 3. Manual pin (less common)
    let boxed = Box::new(MyData { value: 42 });
    let _pinned = Pin::new(boxed);

    // You can still access the data
    println!("Value: {}", heap_pinned.value);
}

