struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position
            .into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| (p, (target - p) as f64 / s as f64))
            .collect();

        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0;
        let mut time_to_reach = 0.0;

        for &(_, time) in &cars {
            if time > time_to_reach {
                fleets += 1;
                time_to_reach = time;
            }
        }

        fleets
    }
}

fn main() {
    let target = 12;
    let position = vec![10,8,0,5,3];
    let speed = vec![2,4,1,1,3];

    println!("{}", Solution::car_fleet(target, position, speed));
}