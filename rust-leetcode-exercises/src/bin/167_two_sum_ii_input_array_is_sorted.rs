struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if numbers[left] + numbers[right] > target {
                right -= 1;
            } else {
                left += 1;
            }
        }

        return vec![(left + 1) as i32, (right + 1) as i32];
    }
}

fn main() {
    let numbers = vec![2,7,11,15];
    let target = 9;

    println!("{:?}", Solution::two_sum(numbers, target));
}