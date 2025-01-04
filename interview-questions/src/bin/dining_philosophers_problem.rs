use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    const NUM_DINERS: usize = 5;
    let forks = Arc::new((0..NUM_DINERS).map(|_| Mutex::new(())).collect::<Vec<_>>());

    let mut handles = vec![];

    for i in 0..NUM_DINERS {
        let forks = Arc::clone(&forks);

        // Each diner is represented by a thread
        let handle = thread::spawn(move || {
            let left_fork = i;
            let right_fork = (i + 1) % NUM_DINERS;

            for _ in 0..3 {
                // Thinking
                println!("Diner {} is thinking.", i);
                thread::sleep(Duration::from_secs(1));

                // Try to pick up the forks
                let _left = forks[left_fork].lock().unwrap(); // Lock the left fork
                println!("Diner {} picked up the left fork.", i);

                let _right = forks[right_fork].lock().unwrap(); // Lock the right fork
                println!("Diner {} picked up the right fork.", i);

                // Eating
                println!("Diner {} is eating.", i);
                thread::sleep(Duration::from_secs(2));

                // Release forks (automatically when `_left` and `_right` go out of scope)
                println!("Diner {} put down both forks.", i);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All diners have finished.");
}
