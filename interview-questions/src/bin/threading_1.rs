/*
Code takes 4 element vector, spawns 4 threads and adds 5 to each element. 
The idea is to simulate delay inside each thread during the operation. 
That is accomplished with thread::sleep() function
*/

use rand::random;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Works on vector of tasks and increments them");
    task_increment();
}

fn task_increment() {
    let glb_task = Arc::new(Mutex::new(vec![7, 6, 2, 5]));
    let mut hdls = vec![];

    for idx in 0..4 {
        let cln_vec = Arc::clone(&glb_task);
        let hdl = thread::spawn(move || {
            let mut loc_vec = cln_vec.lock().unwrap();
            loc_vec[idx] += 5;
            let waiter = random::<u64>() % 100 / 10;
            println!("Waiting for {} sec", waiter);
            thread::sleep(Duration::from_secs(waiter));
        });
        hdls.push(hdl);
    }
    for d in hdls {
        d.join().unwrap()
    }

    println!("{:?}", glb_task.lock().unwrap())
}
