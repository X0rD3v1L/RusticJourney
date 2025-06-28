struct Solution;

impl Solution {
    pub fn can_ship(weights: Vec<i32>, capacity: i32, days: i32) -> bool {
        let mut total = 0;
        let mut days_needed = 1;

        for &weight in &weights {
            if total + weight > capacity {
                days_needed += 1;
                total = weight; // Start a new day with the current weight
                if days_needed > days {
                    return false; // Exceeded the number of days allowed
                }
            } else {
                total += weight;
            }
        }

        true // All weights can be shipped within the given days
    }
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut low = *weights.iter().max().unwrap();
        let mut high: i32 = weights.iter().sum();

        while low < high {
            let mid = low + (high - low) / 2;
            if Self::can_ship(weights.clone(), mid, days) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

fn main() {
    let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let days = 5;
    let result = Solution::ship_within_days(weights, days);
    println!("Minimum capacity to ship within {} days: {}", days, result);
}