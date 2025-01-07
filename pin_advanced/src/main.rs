use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};


// Basic Usage: The Pin wrapper ensures our Timer stays in one place in memory
// Safety: Even though this example is simple, Pin is required by the Future trait
// Real World: In real async code, futures often contain references to their own data

// A simple timer future
struct Timer {
    expiration: Instant,
}

impl Timer {
    fn new(duration: Duration) -> Self {
        Timer {
            expiration: Instant::now() + duration,
        }
    }
}

// Implement Future for our Timer
impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[async_std::main]
async fn main() {
    // Create a timer for 1 second
    let timer = Timer::new(Duration::from_secs(1));
    
    // Wait for the timer
    timer.await;
    println!("Timer finished!");
}