use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort_unstable();

            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }
}

fn main() {
    let strs = vec![
        "eat".to_string(), "tea".to_string(), "tan".to_string(),
        "ate".to_string(), "nat".to_string(), "bat".to_string(),
    ];
    println!("{:?}", Solution::group_anagrams(strs));
}

/*
Time Complexity:
Sorting each word takes 
O(klogk) (where k is the word length).

Iterating through n words results in 
O(n⋅klogk).

Space Complexity:
O(n⋅k) for storing the groups in HashMap.
*/

/*
use std::collections::HashMap;

impl Solution {
   pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for string in strs {
            let mut count = vec![0; 26];
            for c in string.chars() {
                let i = (c as u8) - ('a' as u8);
                count[i as usize] += 1;
            }
            map.entry(count)
                .and_modify(|f| {
                    f.push(string.clone());
                })
                .or_insert(vec![string]);
        }
        map.into_values().collect()
    }
}
*/