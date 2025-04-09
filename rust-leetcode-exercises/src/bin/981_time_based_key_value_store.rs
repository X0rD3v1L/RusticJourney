use std::collections::HashMap;

struct TimeMap {
    time_map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            time_map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.time_map.entry(key).or_insert(Vec::new());
        entry.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(entries) = self.time_map.get(&key) {
            let mut left = 0;
            let mut right = entries.len() - 1;
            let mut res = "";

            while left <= right {
                let mid = left + (right - left) / 2;
                let (mid_ts, ref mid_val) = entries[mid];
                if mid_ts == timestamp {
                    return mid_val.clone();
                } else if mid_ts < timestamp {
                    res = mid_val;
                    left = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            }

            return res.to_string();
        }

        "".to_string()
    }
}

fn main() {
    let mut obj = TimeMap::new();

    obj.set("foo".to_string(), "bar".to_string(), 1);
    println!("{}", obj.get("foo".to_string(), 1)); // "bar"
    println!("{}", obj.get("foo".to_string(), 3)); // "bar"

    obj.set("foo".to_string(), "bar2".to_string(), 4);
    println!("{}", obj.get("foo".to_string(), 4)); // "bar2"
    println!("{}", obj.get("foo".to_string(), 5)); // "bar2"

    println!("{}", obj.get("foo".to_string(), 0)); // ""
    println!("{}", obj.get("baz".to_string(), 1)); // ""
}
