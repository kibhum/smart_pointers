use std::sync::{Arc, Mutex};
use std::thread;

// Utilizing Mutex<T> to protect shared data in a multithreaded environment
// Locking and unlocking a Mutex<T> to ensure exclusive access and prevent data races
fn protect_shared_data() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count: {}", *counter.lock().unwrap());
}
