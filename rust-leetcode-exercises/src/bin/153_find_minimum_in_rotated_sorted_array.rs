use std::cmp::min;

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut ans = i32::MAX;
        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[low] <= nums[mid] {
                ans = min(ans, nums[low]);
                low = mid + 1;
            } else {
                ans = min(ans, nums[mid]);
                high = mid - 1;
            }
        }
        ans
    }
}

fn main() {
    let nums = vec![3,4,5,1,2];
    println!("{}", Solution::find_min(nums));
}