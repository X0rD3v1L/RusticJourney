struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut count_zeroes = 0;
        let mut product = 1;

        for num in &nums {
            if num == &0 {
                count_zeroes += 1;
                continue
            }
            product = product * num;
        }

        let mut ans = vec![0; n];
        if count_zeroes == 0 {
            for i in 0..n {
                ans[i] = product / nums[i];
            }
        }
        else if count_zeroes == 1 {
            for i in 0..n {
                if nums[i] == 0 {
                    ans[i] = product;
                }
            }
        }

        return ans
    }
}

fn main() {
    let arr = vec![-1, 1, 0, -3, 3];
    println!("{:?}", Solution::product_except_self(arr));
}