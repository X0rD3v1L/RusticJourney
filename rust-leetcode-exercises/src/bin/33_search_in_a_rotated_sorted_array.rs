struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high {
            let mid = (low + high) >> 1;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[low] <= nums[mid] {
                if target >= nums[low] && target <= nums[mid] {
                    high = mid - 1;
                } else{
                    low = mid + 1;
                }
            } else{
                if target >= nums[mid] && target <= nums[high] {
                    low = mid + 1;
                } else{
                    high = mid - 1;
                }
            }
        }
        -1
    }
}

fn main() {
    let nums = vec![4,5,6,7,0,1,2];
    let target = 0;

    println!("{}", Solution::search(nums, target));
}