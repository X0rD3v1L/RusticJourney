use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = nums.iter().take(k as usize).map(|&x| Reverse(x)).collect();
        
        for &num in nums.iter().skip(k as usize) {
            if num > heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        
        heap.peek().unwrap().0
    }
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 3;

    println!("{}", Solution::find_kth_largest(nums, k));
}