pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<u32>) -> u32{
        let mut ans = 0;

        for element in nums {
            ans ^= element;
        }
        ans
    }
}

fn main(){
    let nums = vec![2,2,1];
    let single = Solution::single_number(nums);
    println!("{}", single);
}