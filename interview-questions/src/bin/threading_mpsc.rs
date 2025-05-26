use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Producer thread
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap(); // sends data to receiver
        }
    });

    // Consumer thread (main)
    for received in rx {
        println!("Received: {}", received);
    }
}
