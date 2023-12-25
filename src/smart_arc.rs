use std::sync::Arc;
use std::thread;
// Atomic reference counting for thread-safe shared ownership
fn thread_safe_and_shared_ownership() {
    let data = Arc::new("Hello, world!");
    let mut handles = vec![];
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Data in thread: {}", data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
