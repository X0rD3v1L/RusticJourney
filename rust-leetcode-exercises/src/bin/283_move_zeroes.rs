pub struct Solution;

impl Solution {
    pub fn move_zeroes_to_end(nums: &mut Vec<i32>){
        let mut c = 0;

        for index in 0..nums.len() {
            if nums[index] != 0 {
                let temp = nums[c];
                nums[c] = nums[index];
                nums[index] = temp;
                c += 1; 
            }
        }

        println!("{:?}", nums);
    }
}

fn main(){
    let mut nums = vec![0,1,0,3,12];
    Solution::move_zeroes_to_end(&mut nums);
}