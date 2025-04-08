struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut t = 1_000_000_000;

        while l < t {
            let speed = (l + t) / 2;
            
            //This is a clever and safe way of computing ceil(x / speed) using integer math.
            let time: i32 = piles.iter().map(|x| (x - 1) / speed + 1).sum();

            if time <= h {
                t = speed;
            } else {
                l = speed + 1;
            }
        }

        t
    }
}

fn main() {
    let piles = vec![3,6,7,11];
    let h = 8;

    println!("{}", Solution::min_eating_speed(piles, h));
}