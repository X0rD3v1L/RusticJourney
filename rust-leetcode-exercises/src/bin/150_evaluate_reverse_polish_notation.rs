struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
                "-" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                }
                "*" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                }
                "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a / b);
                }
                num => {
                    let n = num.parse::<i32>().unwrap();
                    stack.push(n);
                }
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    let tokens = vec!["2".to_owned(),"1".to_owned(),"+".to_owned(),"3".to_owned(),"*".to_owned()];

    println!("{}", Solution::eval_rpn(tokens));
}