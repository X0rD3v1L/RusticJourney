struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut vol = 0;
        let mut max_l = 0;
        let mut max_r = 0;

        while l < r {
            if height[l] < height[r] {
                if max_l > height[l] {
                    vol += max_l - height[l];
                } else {
                    max_l = height[l];
                }
                l += 1;
            } else {
                if max_r > height[r] {
                    vol += max_r - height[r];
                } else {
                    max_r = height[r];
                }
                r -= 1;
            }
        }

        return vol;
    }
}

fn main() {
    
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];

    println!("{}", Solution::trap(height));
}

/*
Time complexity - O(n)
Space complexity - O(1)
*/