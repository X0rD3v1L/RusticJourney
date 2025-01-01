use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_price = prices[0];

        for price_index in 1..prices.len() {
            if prices[price_index] < min_price {
                min_price = prices[price_index];
            }
            profit = max(profit, prices[price_index] - min_price);
        }
        profit
    }
}

fn main() {
    let ans = Solution::max_profit(vec![7,1,5,3,6,4]);
    println!("{}", ans);
}