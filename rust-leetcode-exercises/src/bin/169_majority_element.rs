pub struct  Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut element = 0;

        for i in 0..nums.len() {
            if count == 0 {
                element = nums[i];
            }
            if element == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }

        return element;
    }
}

fn main(){
    let nums = vec![2,2,1,1,1,2,2];
    println!("{}", Solution::majority_element(nums));
}