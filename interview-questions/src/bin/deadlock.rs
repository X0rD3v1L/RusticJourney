use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource_1 = Arc::new(Mutex::new(0));
    let resource_2 = Arc::new(Mutex::new(0));

    let res1_clone1 = Arc::clone(&resource_1);
    let res2_clone1 = Arc::clone(&resource_2);

    let thread1 = thread::spawn(move || {
        let _lock1 = res1_clone1.lock().unwrap();
        println!("Thread 1: Locked resource 1");
        thread::sleep(std::time::Duration::from_secs(1));

        let _lock2 = res2_clone1.lock().unwrap();
        println!("Thread 1: Locked resource 2");
    });

    let res1_clone2 = Arc::clone(&resource_1);
    let res2_clone2 = Arc::clone(&resource_2);

    let thread2 = thread::spawn(move || {
        let _lock2 = res2_clone2.lock().unwrap();
        println!("Thread 2: Locked resource 2");
        thread::sleep(std::time::Duration::from_secs(1));

        let _lock1 = res1_clone2.lock().unwrap();
        println!("Thread 2: Locked resource 1");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    /*
    Thread 1: Locked resource 1
    Thread 2: Locked resource 2
    ...

    This is a Deadlock situation as Thread 1 is waiting for Resource 2 and Thread 2 is waiting for Resource 1.
    */

}