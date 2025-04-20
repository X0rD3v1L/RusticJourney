use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A key-value pair container using open addressing with linear probing.
pub struct OpenAddressingHashMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
    capacity: usize,
    load_factor_threshold: f64,
}

impl<K: Eq + Hash + Clone, V: Clone> OpenAddressingHashMap<K, V> {
    /// Creates a new hashmap with default capacity.
    pub fn new() -> Self {
        let capacity = 16;
        Self {
            buckets: vec![None; capacity],
            size: 0,
            capacity,
            load_factor_threshold: 0.7,
        }
    }

    /// Hash a key into an index for the buckets.
    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }

    /// Insert a key-value pair into the map. Overwrites if key already exists.
    pub fn insert(&mut self, key: K, value: V) {
        if (self.size as f64 / self.capacity as f64) >= self.load_factor_threshold {
            self.resize();
        }

        let mut index = self.hash_key(&key);

        loop {
            match &self.buckets[index] {
                Some((existing_key, _)) if existing_key == &key => {
                    self.buckets[index] = Some((key, value));
                    return;
                }
                None => {
                    self.buckets[index] = Some((key, value));
                    self.size += 1;
                    return;
                }
                _ => {
                    index = (index + 1) % self.capacity;
                }
            }
        }
    }

    /// Retrieve a reference to the value associated with the key, if any.
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash_key(key);

        loop {
            match &self.buckets[index] {
                Some((existing_key, value)) if existing_key == key => {
                    return Some(value);
                }
                Some(_) => {
                    index = (index + 1) % self.capacity;
                }
                None => {
                    return None;
                }
            }
        }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Internal function to resize and rehash all keys into a bigger table.
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let old_buckets = std::mem::replace(&mut self.buckets, vec![None; new_capacity]);
        self.capacity = new_capacity;
        self.size = 0;

        for slot in old_buckets.into_iter() {
            if let Some((k, v)) = slot {
                self.insert(k, v);
            }
        }
    }
}

fn main() {
    let mut map = OpenAddressingHashMap::new();

    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("orange", 2);
    map.insert("banana", 10); // overwrite test

    println!("apple => {:?}", map.get(&"apple"));
    println!("banana => {:?}", map.get(&"banana"));
    println!("orange => {:?}", map.get(&"orange"));
    println!("grape => {:?}", map.get(&"grape"));
    println!("Size: {}", map.len());
}
