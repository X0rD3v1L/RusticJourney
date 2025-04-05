pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums : Vec<i32>) -> bool {
        let mut hashmap = HashMap::new();

        for element in nums{
            if hashmap.contains_key(&element) {
                return true;
            }else{
                let count = hashmap.entry(element).or_insert(0);
                *count += 1;
            }
        }
        return false;
    }
}

fn main(){
    let nums = vec![1,2,3,1];
    println!("{}", Solution::contains_duplicate(nums));
}

/*
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set = HashSet::new();

        for element in nums {
            if hash_set.contains(&element) {
                return true;
            }

            hash_set.insert(element);
        }

        return false;
    }
}
*/

/*
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.windows(2).any(|w| w[0] == w[1])
    }
}
*/

/*
HashSet - O(n) time O(n) space
Sorting - O(n log (n)) time O(1) space
*/