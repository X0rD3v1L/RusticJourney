use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.into_iter()
            .for_each(|num| *map.entry(num).or_insert(0) += 1);

        let mut vec: Vec<(i32, i32)> = map.into_iter().collect();

        vec.sort_by(|a, b| b.1.cmp(&a.1));

        vec.iter().take(k as usize).map(|x| x.0).collect()
        
    }
}

fn main() {
    let nums = vec![1, 2, 2, 3, 1];
    println!("{:?}", Solution::top_k_frequent(nums, 2));
}