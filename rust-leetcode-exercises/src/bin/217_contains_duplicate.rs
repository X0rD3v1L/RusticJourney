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