use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use async_std::task;


// Define our custom Fibonacci stream struct
struct Fibonacci {
    current: u64,
    next: u64,
    count: usize,
    max_count: usize,
}

impl Fibonacci {
    fn new(max_count: usize) -> Self {
        Fibonacci {
            current: 0,
            next: 1,
            count: 0,
            max_count,
        }
    }
}

// Implement Stream trait for Fibonacci struct 
// Using Pin for self-referential structs
// Managing state within a stream
impl Stream for Fibonacci {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.count >= self.max_count {
            return Poll::Ready(None);
        }
    

    let next_number = self.current;
    let new_next = self.current + self.next;
    self.current = self.next;
    self.next = new_next;
    self.count += 1;

    Poll::Ready(Some(next_number))
    }
}


#[async_std::main]
async fn main() {
    use futures::StreamExt; // For next() method

    // Create a Fibonacci stream that generates 10 numbers
    // Controlling when the stream ends
    let mut fib = Fibonacci::new(10);

    // Consume the stream
    while let Some(number) = fib.next().await {
        println!("Fibonacci number: {}", number);
    }
}