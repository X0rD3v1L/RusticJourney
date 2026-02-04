use std::cmp::min;

fn max_profit(prices: &[i32]) -> i32 {
    let mut min_so_far = i32::MAX;
    let mut best_profit = 0;

    for p in prices {
        let diff = p - min_so_far;

        if diff > best_profit {
            best_profit = diff;
        }

        min_so_far = min(min_so_far, *p);
    }
    best_profit
}
//Time complexity is O(n) since we traverse the array once.
//Space complexity is O(1) since we use only a fixed amount of extra space
fn main() {
    let prices = vec![7,1,5,3,6,4];
    let ans = max_profit(&prices);
    println!("{}", ans);
}