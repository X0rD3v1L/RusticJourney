use std::{cmp::max, collections::HashSet};

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hash_set: HashSet<_> = nums.into_iter().collect();
        let mut longest_streak = 0;
        
        for &num in &hash_set {
            if !hash_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while hash_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest_streak = max(longest_streak, current_streak);
            }
        }

        longest_streak
    }
}

fn main() {
    let nums = vec![100,4,200,1,3,2];

    println!("{}", Solution::longest_consecutive(nums));

}