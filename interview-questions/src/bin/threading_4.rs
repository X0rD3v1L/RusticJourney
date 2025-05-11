/*
Write a rust code where 3 threads are reading a single variable 
and one thread is writing to that same variable.
*/

use parking_lot::RwLock;
use rand::{rng, Rng};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Read-Write concurrency demonstration");
    let rw_main = Arc::new(RwLock::new(0));
    let running = Arc::new(AtomicBool::new(true));
    let mut rthls = vec![];

    // Three threads reading concurrently
    for _ in 0..3 {
        let cl_rw = Arc::clone(&rw_main);
        let is_running = Arc::clone(&running);
        let rth = thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {
                let reader = cl_rw.read();
                println!("Reader read: {reader}");
                thread::sleep(Duration::from_millis(50)); // Simulate some work
            }
            println!("Reader stopped.");
        });
        rthls.push(rth);
    }

    // Single thread writing concurrently
    let cl_w = Arc::clone(&rw_main);
    let is_running = Arc::clone(&running);
    let wth = thread::spawn(move || {
        while is_running.load(Ordering::Relaxed) {
            let rval = rng().random_range(10..566);
            {
                let mut writer = cl_w.write();
                *writer = rval;
                println!("Writer wrote: {rval}");
            }
            thread::sleep(Duration::from_millis(100)); // Simulate some work
        }
        println!("Writer stopped.");
    });

    // Let the threads run for 2 seconds
    thread::sleep(Duration::from_millis(500));

    // Stop the threads
    running.store(false, Ordering::Relaxed);

    // Wait for all threads to finish
    for th in rthls {
        th.join().unwrap();
    }
    wth.join().unwrap();

    println!("Final rwlock val: {:?}", *rw_main.read());
}
