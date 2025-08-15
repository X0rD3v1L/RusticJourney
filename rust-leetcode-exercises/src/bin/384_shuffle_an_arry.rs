use rand::distr::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::rng;

struct Solution {
    original: Vec<i32>,
    shuffled: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution {
            original: nums.clone(),
            shuffled: nums,
            rng: rng(),
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.shuffled = self.original.clone();
        self.original.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let len = self.shuffled.len();
        for i in 0..len - 1 {
            let dist = Uniform::new_inclusive(i, len - 1).expect("Invalid range");
            let j = dist.sample(&mut self.rng);
            self.shuffled.swap(i, j);
        }
        self.shuffled.clone()
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let mut solution = Solution::new(nums);

    println!("Original: {:?}", solution.reset());
    println!("Shuffled: {:?}", solution.shuffle());
    println!("Reset: {:?}", solution.reset());
}
