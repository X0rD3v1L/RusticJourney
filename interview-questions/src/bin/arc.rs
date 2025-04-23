use std::sync::{Arc, Mutex};
use std::thread;

/*
Arc: Used for multi-threaded shared ownership.
*/
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    };
    
    for handle in handles {
        handle.join().unwrap();
    };
    
    println!("Result {}", *counter.lock().unwrap());
}