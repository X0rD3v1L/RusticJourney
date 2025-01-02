use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    usage: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            usage: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&value) = self.map.get(&key) {
            // Update usage
            self.usage.retain(|&k| k != key);
            self.usage.push_back(key);
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            // Update existing key
            self.usage.retain(|&k| k != key);
        } else if self.map.len() == self.capacity {
            // Evict least recently used key
            if let Some(lru) = self.usage.pop_front() {
                self.map.remove(&lru);
            }
        }
        // Insert key-value and update usage
        self.map.insert(key, value);
        self.usage.push_back(key);
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    println!("{:?}", cache.get(1)); // Some(10)
    cache.put(3, 30); // Evicts key 2
    println!("{:?}", cache.get(2)); // None
    cache.put(4, 40); // Evicts key 1
    println!("{:?}", cache.get(1)); // None
    println!("{:?}", cache.get(3)); // Some(30)
    println!("{:?}", cache.get(4)); // Some(40)
}
