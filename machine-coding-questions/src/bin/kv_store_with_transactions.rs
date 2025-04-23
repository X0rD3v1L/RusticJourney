use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
struct KeyNotFound;

impl fmt::Display for KeyNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key not found in store")
    }
}

struct KVStore {
    stack: Vec<HashMap<String, u32>>,
}

impl KVStore {
    fn new() -> Self {
        KVStore {
            stack: vec![HashMap::new()],
        }
    }

    fn set(&mut self, key: &str, value: u32) {
        self.stack.last_mut().unwrap().insert(key.to_owned(), value);
    }

    fn get(&self, key: &str) -> Result<u32, KeyNotFound> {
        for map in self.stack.iter().rev() {
            if let Some(&value) = map.get(key) {
                return Ok(value);
            }
        }
        Err(KeyNotFound)
    }

    fn delete(&mut self, key: &str) -> Result<(), KeyNotFound> {
        for map in self.stack.iter_mut().rev() {
            if map.contains_key(key) {
                map.remove(key);
                return Ok(());
            }
        }
        Err(KeyNotFound)
    }

    fn begin(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn commit(&mut self) {
        if self.stack.len() <= 1 {
            return; // nothing to commit
        }

        let top = self.stack.pop().unwrap();
        let parent = self.stack.last_mut().unwrap();

        for (k, v) in top {
            parent.insert(k, v);
        }
    }

    fn rollback(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }
}

fn main() {
    let mut kv = KVStore::new();

    kv.set("1", 3);
    assert_eq!(kv.get("1"), Ok(3));
    assert_eq!(kv.get("2"), Err(KeyNotFound));
    assert_eq!(kv.delete("3"), Err(KeyNotFound));
    kv.begin();
    kv.set("2", 4);
    assert_eq!(kv.get("1"), Ok(3));
    assert_eq!(kv.get("2"), Ok(4));
    kv.commit();
    assert_eq!(kv.get("2"), Ok(4));

    kv.begin();
    kv.set("3", 5);
    assert_eq!(kv.get("3"), Ok(5));
    kv.rollback();
    assert_eq!(kv.get("3"), Err(KeyNotFound));

    kv.begin();
    kv.set("4", 7);
    kv.begin();
    kv.set("5", 9);
    kv.commit();
    assert_eq!(kv.get("4"), Ok(7));
    assert_eq!(kv.get("5"), Ok(9));
    kv.rollback();
    assert_eq!(kv.get("4"), Err(KeyNotFound));
    assert_eq!(kv.get("5"), Err(KeyNotFound));

    println!("All tests passed!");
}
