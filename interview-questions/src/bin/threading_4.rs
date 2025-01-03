/*
Write a rust code where 3 threads are reading a single variable 
and one thread is writing to that same variable.
*/

use parking_lot::RwLock;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread;

fn main() {
    println!("Read write");
    let rw_main = Arc::new(RwLock::new(0));
    let mut rthls = vec![];
    // three threads reading
    for _ind in 0..3 {
        let cl_rw = Arc::clone(&rw_main);
        let rth = thread::spawn(move || {
            let reader = cl_rw.read();
            println!("Reader read: {reader}");
        });
        rthls.push(rth);
    }
    // single threads writing
    let cl_w = Arc::clone(&rw_main);
    thread::spawn(move || {
        let rval = thread_rng().gen_range(10..566);
        println!("writing {rval}");
        *cl_w.write() = rval;
    })
    .join()
    .unwrap();
    for th in rthls {
        th.join().unwrap();
    }
    println!("Final rwlock val: {:?}", rw_main)
}
