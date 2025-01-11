use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    // 1. Basic Thread Creation
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
        thread::sleep(Duration::from_millis(1000));
        println!("Thread finished!");
    });

    println!("Main thread continues...");
    handle.join().unwrap(); // Wait for thread to finish
    
    // 2. Sharing Data Between Threads using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
            thread::sleep(Duration::from_millis(500));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    // 3. Channels for Thread Communication
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["Hello", "From", "Thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    
    // Receive messages
    for received in rx {
        println!("Got: {}", received);
    }
}