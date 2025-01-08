

// *************************Why We Need It*************************:
// Prevents moving data that shouldn't be moved
// Essential for self-referential structs
// Required for async/await (futures often contain self-references

// *************************What Pin Guarantees-I*************************:
let pinned = Box::pin(my_data);
// ✓ Data won't move in memory
// ✓ Safe to create self-references
// ✓ Can still access data through references
// *************************What Pin Guarantees-II*************************:
// Pinning ensures that the data is not moved to a different memory location
// Pinning is required for any type that implements the Future trait
// Pinning is also required for any type that implements the Stream trait
// Pinning is required for any type that implements the Sink trait
// Pinning is required for any type that implements the Future trait

// *************************Common Use Cases*************************
// Async/await
// Self-referential structs
// Futures and Streams
// Custom async types

// *************************Two Main Ways to Pin*************************:
// 1. Pin to heap (most common)
let heap_pinned = Box::pin(my_data);

// 2. Pin to stack (less common)
use std::pin::pin;
let stack_pinned = pin!(my_data);

//*************************The Unpin Trait*************************:

// Most types can be safely moved even when pinned
struct Regular(i32);  // Automatically implements Unpin

// To prevent moving, explicitly opt-out of Unpin
struct NoMove(i32, PhantomPinned);  // Won't implement Unpin