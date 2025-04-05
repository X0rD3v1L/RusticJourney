struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let iter = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();

    println!("{}", Solution::is_palindrome(s));
}