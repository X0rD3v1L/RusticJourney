use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();

        for index in 0..nums.len() {
            let complement = target - nums[index];
            if let Some(&prev_index) = hashmap.get(&complement) {
                return vec![prev_index as i32, index as i32];
            }else {
                hashmap.insert(nums[index], index);
            }
        }

        vec![]
    }
}

fn main() {
    let nums = vec![1,2,3,1];
    println!("{:?}", Solution::two_sum(nums, 5));
}

/*
Time - O(n)
Space - O(n)
*/