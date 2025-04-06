struct Solution;

impl Solution {
    fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        nums.sort_unstable();
        let length = nums.len();

        for i in 0..length - 2 {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = length - 1;
            while left < right {
                let total = nums[i] + nums[left] + nums[right];
                if total < 0 {
                    left += 1;
                } else if total > 0 {
                    right -= 1;
                } else {
                    ans.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left = left + 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right = right - 1;
                    }
                    left = left + 1;
                    right = right - 1;
                }
            }
        }
        return ans;
    }
}
fn main() {
    let nums = vec![-1,0,1,2,-1,-4];

    println!("{:?}", Solution::three_sum(nums));
}