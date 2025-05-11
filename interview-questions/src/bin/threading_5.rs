/*
Write rust code where 4 threads are spawned, which take different time to complete. 
Implement a logic where one of the threads wait for other threads to complete and then execute together.
*/

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use rand::{rng, Rng};

fn main() {
    let barrier = Arc::new(Barrier::new(4)); // 4 threads will synchronize here
    let mut handles = vec![];

    for i in 0..4 {
        let cbarrier = Arc::clone(&barrier);

        // Spawn threads
        let handle = thread::spawn(move || {
            if i == 3 {
                // Thread 3: Wait for others
                println!("Thread {i} is waiting for others to finish.");
                cbarrier.wait(); // Wait for other threads to complete
                println!("Thread {i} starts execution after others finished.");
            } else {
                // Other threads: Perform some task
                let delay = rng().random_range(1..5); // Random sleep time (1-4 seconds)
                println!("Thread {i} working for {delay} seconds.");
                thread::sleep(Duration::from_secs(delay));
                println!("Thread {i} finished work.");
                cbarrier.wait(); // Wait at the barrier
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have completed.");
}
