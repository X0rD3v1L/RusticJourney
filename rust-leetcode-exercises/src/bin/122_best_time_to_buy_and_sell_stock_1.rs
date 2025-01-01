pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        //Solved through greedy approach
        for price_index in 1..prices.len() {
            if prices[price_index] > prices[price_index - 1] {
                profit += prices[price_index] - prices[price_index - 1];
            }
        }
        profit
    }
}

fn main() {
    let ans = Solution::max_profit(vec![7,1,5,3,6,4]);
    println!("{}", ans);
}