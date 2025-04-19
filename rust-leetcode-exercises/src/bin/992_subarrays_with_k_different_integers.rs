use std::{collections::HashMap};

struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::count_subarrays_with_goal(&nums, k) - Self::count_subarrays_with_goal(&nums, k - 1)
    }

    fn count_subarrays_with_goal(nums: &Vec<i32>, goal: i32) -> i32 {
        let (mut l, mut r, mut count) = (0,0,0);
        let mut map = HashMap::new();
        let n = nums.len();

        while r < n {
            *map.entry(nums[r]).or_insert(0) += 1;

            while map.len() > goal as usize {
                let entry = map.get_mut(&nums[l]).unwrap();
                *entry -= 1;

                if *entry == 0 {
                    map.remove(&nums[l]);
                }
                l += 1;
            }
            count += (r - l + 1) as i32;
            r += 1;
        }
        count
    }
}

fn main() {
    let nums = vec![1,2,1,2,3];
    let k = 2;
    println!("{}", Solution::subarrays_with_k_distinct(nums, k));
}
