pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_index = nums[0] as usize;

        for index in 1..nums.len() {
            if index > max_index {
                return false;
            }
            max_index = max_index.max(nums[index] as usize + index);
        }
        true
    }
}

fn main() {
    let nums = vec![2,3,1,1,4];
    println!("{}", Solution::can_jump(nums));
}