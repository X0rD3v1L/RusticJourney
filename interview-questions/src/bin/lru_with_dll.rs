// LRU Cache using Doubly Linked List and HashMap in Rust
use std::collections::HashMap;
use std::ptr;

// Node structure for DLL
struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, value: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

// LRU Cache structure
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self {
            capacity,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    // Add node to the front (most recently used)
    fn add_to_front(&mut self, node: *mut Node) {
        unsafe {
            (*node).next = (*self.head).next;
            (*(*self.head).next).prev = node;
            (*self.head).next = node;
            (*node).prev = self.head;
        }
    }

    // Remove a node from DLL
    fn remove_node(&mut self, node: *mut Node) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&node) = self.map.get(&key) {
            self.remove_node(node);
            self.add_to_front(node);
            unsafe { (*node).value }
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&node) = self.map.get(&key) {
            self.remove_node(node);
            unsafe { (*node).value = value; }
            self.add_to_front(node);
        } else {
            if self.map.len() == self.capacity {
                let lru = unsafe { (*self.tail).prev };
                self.remove_node(lru);
                let lru_key = unsafe { (*lru).key };
                self.map.remove(&lru_key);
            }
            let new_node = Node::new(key, value);
            self.add_to_front(new_node);
            self.map.insert(key, new_node);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2); // Capacity of 2
    cache.put(1, 1);
    cache.put(2, 2);
    println!("get(1): {}", cache.get(1)); // returns 1
    cache.put(3, 3); // evicts key 2
    println!("get(2): {}", cache.get(2)); // returns -1
    cache.put(4, 4); // evicts key 1
    println!("get(1): {}", cache.get(1)); // returns -1
    println!("get(3): {}", cache.get(3)); // returns 3
    println!("get(4): {}", cache.get(4)); // returns 4
}
