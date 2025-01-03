/*
Write a rust program that spawns Two threads. Declare one common variable. 
Thread 1 will increment the variable by certain value, and 
Thread 2 will decrement the variable by certain value.
*/

use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread;

fn main() {
    println!("Two thread mod one variable");
    let mut ctr = 32;

    let (s, r): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let c_inc = s.clone();
    thread::spawn(move || {
        c_inc.send(15).unwrap();
    })
    .join()
    .unwrap();

    let c_dec = s.clone();
    thread::spawn(move || {
        c_dec.send(-5).unwrap();
    })
   .join()
    .unwrap();

    let crx = Arc::clone(&Arc::new(&r));
    for _ in 0..2 {
        let rcv_val = crx.recv().unwrap();
        ctr += rcv_val;
    }

    println!("The counter value is {ctr}");
}