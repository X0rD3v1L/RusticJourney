/*
Design a time-based two-tier storage structure that:

Input: Accepts objects with (Key, Data) pairs to store.

Storage:
Buffer 1 (B1): Fixed-size Buffer of capacity N. New objects go here first.
Buffer 2 (B2): Unlimited-size Buffer. Objects move here when their TTL expires.

Time-to-Live (TTL): Each object has a TTL. When an object in B1 reaches its TTL:
It is moved from B1 to B2.
B1 must not exceed capacity N (decide behavior if full).

Retrieval: Implement get(Key) that:
Finds the key in either buffer.
Returns the associated data (or null if not found).
Should be efficient. - Should Reset the TTL when accessed
*/

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread::sleep;

#[derive(Clone)]
struct Entry {
    data: String,
    expires_at: Instant,
}

struct Store {
    b1: HashMap<String, Entry>,
    order: VecDeque<String>,
    b2: HashMap<String, Entry>,
    capacity: usize,
    ttl: Duration,
}

impl Store {
    fn new(capacity: usize, ttl_secs: u64) -> Self {
        Self {
            b1: HashMap::new(),
            order: VecDeque::new(),
            b2: HashMap::new(),
            capacity,
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn expire(&mut self) {
        let now = Instant::now();

        while let Some(front) = self.order.front() {
            if let Some(entry) = self.b1.get(front) {
                if entry.expires_at > now {
                    break;
                }
            }

            let key = self.order.pop_front().unwrap();
            if let Some(entry) = self.b1.remove(&key) {
                self.b2.insert(key, entry);
            }
        }
    }

    fn put(&mut self, key: String, value: String) {
        self.expire();

        if self.b1.len() >= self.capacity {
            if let Some(old) = self.order.pop_front() {
                if let Some(entry) = self.b1.remove(&old) {
                    self.b2.insert(old, entry);
                }
            }
        }

        let entry = Entry {
            data: value,
            expires_at: Instant::now() + self.ttl,
        };

        self.b1.insert(key.clone(), entry);
        self.order.push_back(key);
    }

    fn get(&mut self, key: &str) -> Option<(String, &'static str)> {
        self.expire();

        if let Some(entry) = self.b1.get_mut(key) {
            entry.expires_at = Instant::now() + self.ttl;

            self.order.retain(|k| k != key);
            self.order.push_back(key.to_string());

            return Some((entry.data.clone(), "B1"));
        }

        if let Some(entry) = self.b2.get(key) {
            return Some((entry.data.clone(), "B2"));
        }

        None
    }
}

fn print_get(store: &mut Store, key: &str) {
    match store.get(key) {
        Some((v, buf)) => println!("get({}) -> {} from {}", key, v, buf),
        None => println!("get({}) -> null", key),
    }
}

fn main() {
    let mut store = Store::new(2, 3);

    println!("Inserting A, B");
    store.put("A".into(), "Apple".into());
    store.put("B".into(), "Banana".into());

    println!("Access A (resets TTL)");
    print_get(&mut store, "A");

    println!("Sleeping 4 seconds...");
    sleep(Duration::from_secs(4));

    println!("Insert C");
    store.put("C".into(), "Cherry".into());

    println!("Current lookups:");
    print_get(&mut store, "A");
    print_get(&mut store, "B");
    print_get(&mut store, "C");

    println!("Sleeping 4 seconds...");
    sleep(Duration::from_secs(4));

    println!("Final lookups:");
    print_get(&mut store, "A");
    print_get(&mut store, "B");
    print_get(&mut store, "C");
}
