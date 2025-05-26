/*
Write a rust program that spawns Two threads. Declare one common variable. 
Thread 1 will increment the variable by certain value, and 
Thread 2 will decrement the variable by certain value.
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(32));

    let counter_inc = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        let mut num = counter_inc.lock().unwrap();
        *num += 15;
    });

    let counter_dec = Arc::clone(&counter);
    let t2 = thread::spawn(move || {
        let mut num = counter_dec.lock().unwrap();
        *num -= 5;
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("The counter value is {}", *counter.lock().unwrap());
}
