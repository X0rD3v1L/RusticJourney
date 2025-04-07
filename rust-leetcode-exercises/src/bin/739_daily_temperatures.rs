struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let mut result = vec![0; temperatures.len()];

        for i in (0..temperatures.len()).rev() {
            while let Some(&top) = stack.last() {
                if temperatures[i] >= temperatures[top] {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&top) = stack.last() {
                result[i] = (top - i) as i32;
            } else {
                result[i] = 0;
            }

            stack.push(i);
        }

        result
    }
}


fn main() {
    let temperatures = vec![73,74,75,71,69,72,76,73];
    println!("{:?}", Solution::daily_temperatures(temperatures));
}

/*
Time complexity - O(n)
Space complexity - O(n)
*/