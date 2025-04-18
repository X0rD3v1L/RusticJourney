struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];

            if max_sum < sum {
                max_sum = sum;
            }

            if sum < 0 {
                sum = 0;
            }
        }
        max_sum
    }
}

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];

    println!("{}", Solution::max_sub_array(nums));
}
/*
Time complexity - O(n)
Space complexity - O(1)
*/