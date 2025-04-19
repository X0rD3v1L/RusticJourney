struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_so_far = i32::MAX;
        let mut best_profit = 0;
        let mut min_index = 0;
        let mut result = vec![0, 0];

        for (i, &price) in prices.iter().enumerate() {
            let diff = price - min_so_far;

            if diff > best_profit {
                best_profit = diff;
                result = vec![min_index, i]; // buy at min_index, sell at i
            }

            if price < min_so_far {
                min_so_far = price;
                min_index = i;
            }
        }
        println!("{:?}", result);
        best_profit
    }
}

fn main() {
    let ans = Solution::max_profit(vec![7,1,5,3,6,4]);
    println!("{}", ans);
}