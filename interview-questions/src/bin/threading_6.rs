//given 10 threads, print 1-100 in order so that thread 1 prints 1,11,21 etc
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let max_number = 100;
    let num_threads = 10;

    // Shared state: a counter wrapped in an Arc<Mutex>
    let counter = Arc::new(Mutex::new(1));

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut current = thread_id + 1;

            while current <= max_number {
                // Lock the counter for printing in order
                let mut num = counter_clone.lock().unwrap();

                // Print only if the current number matches the expected one
                if *num == current {
                    println!("Thread {}: {}", thread_id + 1, current);
                    *num += 1;
                    current += num_threads;
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
