/*
Implement rust code for bounded producer and consumer threads 
which put and take objects into a fixed size vector.
*/
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use rand::{rng, Rng};

const BUFFER_SIZE: usize = 5;

fn producer(shared: Arc<(Mutex<Vec<i32>>, Condvar)>) {
    let (lock, cvar) = &*shared;
    loop {
        let mut buffer = lock.lock().unwrap();

        // Wait while buffer is full
        while buffer.len() == BUFFER_SIZE {
            println!("WAITING - BUFFER SIZE IS FULL");
            buffer = cvar.wait(buffer).unwrap();
        }

        let val = rng().random_range(15..57);
        println!("Producing: {}", val);
        buffer.push(val);

        // Notify the consumer
        cvar.notify_all();
        drop(buffer);

        thread::sleep(Duration::from_secs(1));
    }
}

fn consumer(shared: Arc<(Mutex<Vec<i32>>, Condvar)>) {
    let (lock, cvar) = &*shared;
    loop {
        let mut buffer = lock.lock().unwrap();

        // Wait while buffer is empty
        while buffer.is_empty() {
            println!("WAITING - BUFFER SIZE IS EMPTY");
            buffer = cvar.wait(buffer).unwrap();
        }

        let val = buffer.remove(0);
        println!("Consuming: {}", val);

        // Notify the producer
        cvar.notify_all();
        drop(buffer);

        thread::sleep(Duration::from_secs(2));
    }
}

fn main() {
    println!("Bounded Producer-Consumer...");

    let shared = Arc::new((Mutex::new(vec![]), Condvar::new()));

    let shared_producer = Arc::clone(&shared);
    let shared_consumer = Arc::clone(&shared);

    let producer_handle = thread::spawn(move || {
        producer(shared_producer);
    });

    let consumer_handle = thread::spawn(move || {
        consumer(shared_consumer);
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
