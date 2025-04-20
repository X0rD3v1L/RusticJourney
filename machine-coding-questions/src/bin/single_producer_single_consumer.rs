use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

struct Queue<T> {
    inner: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn producer(&self) -> Producer<T> {
        Producer { queue: Arc::clone(&self.inner) }
    }

    fn consumer(&self) -> Consumer<T> {
        Consumer { queue: Arc::clone(&self.inner) }
    }
}

struct Producer<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Producer<T> {
    fn push(&self, val: T) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(val);
    }
}

struct Consumer<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Consumer<T> {
    fn pop(&self) -> Option<T> {
        let mut q = self.queue.lock().unwrap();
        q.pop_front()
    }
}

fn main() {
    let queue = Queue::new();
    let producer = queue.producer();
    let consumer = queue.consumer();

    let handle = thread::spawn(move || {
        for i in 0..5 {
            producer.push(i);
            println!("Produced {}", i);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let consumer_handle = thread::spawn(move || {
        for _ in 0..5 {
            while let Some(val) = consumer.pop() {
                println!("Consumed {}", val);
            }
            std::thread::sleep(std::time::Duration::from_millis(70));
        }
    });

    handle.join().unwrap();
    consumer_handle.join().unwrap();
}
