use std::collections::{HashMap, BTreeMap};

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    value_map: HashMap<i32, (i32, i32)>, // key -> (value, frequency)
    freq_map: HashMap<i32, BTreeMap<i32, ()>>, // frequency -> (key, ())
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            min_freq: 0,
            value_map: HashMap::new(),
            freq_map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&(value, freq)) = self.value_map.get(&key) {
            self.update_frequency(key, freq);
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&(_old_value, freq)) = self.value_map.get(&key) {
            self.value_map.insert(key, (value, freq));
            self.update_frequency(key, freq);
        } else {
            if self.value_map.len() == self.capacity {
                if let Some((&evict_key, _)) = self.freq_map.get_mut(&self.min_freq).unwrap().iter().next() {
                    self.freq_map.get_mut(&self.min_freq).unwrap().remove(&evict_key);
                    self.value_map.remove(&evict_key);
                }
            }
            self.value_map.insert(key, (value, 1));
            self.freq_map.entry(1).or_default().insert(key, ());
            self.min_freq = 1;
        }
    }

    fn update_frequency(&mut self, key: i32, freq: i32) {
        self.freq_map.get_mut(&freq).unwrap().remove(&key);
        if self.freq_map.get(&freq).unwrap().is_empty() {
            self.freq_map.remove(&freq);
            if self.min_freq == freq {
                self.min_freq += 1;
            }
        }
        let new_freq = freq + 1;
        self.value_map.insert(key, (self.value_map[&key].0, new_freq));
        self.freq_map.entry(new_freq).or_default().insert(key, ());
    }
}

fn main() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    println!("get(2): {:?}", cache.get(2)); // Output: Some(20)
    cache.put(3, 30); // Evicts key 1 (LFU)
    println!("get(1): {:?}", cache.get(1)); // Output: None
    println!("get(3): {:?}", cache.get(3)); // Output: Some(30)
}
