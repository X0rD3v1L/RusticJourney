struct Solution;

impl Solution {
    pub fn search(arr: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = arr[mid as usize];

            if mid_val == target {
                return mid;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

fn main() {
    let arr = vec![-1,0,3,5,9,12];
    let target = 9;
    println!("{}", Solution::search(arr, target));
}