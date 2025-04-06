use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            water = max(water, min(height[left], height[right]) * (right as i32 - left as i32));
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        return water;
    }
}

fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];

    println!("{}", Solution::max_area(height));
}