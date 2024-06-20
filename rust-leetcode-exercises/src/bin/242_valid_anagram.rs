pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        if s.len() != t.len() {
            return false;
        }

        let mut freq = [0; 26];

        for c in s.chars() {
            freq[c as usize - 'a' as usize] += 1;
        }

        for c in t.chars() {
            freq[c as usize - 'a' as usize] -= 1;
        }

        freq.iter().all(|&count| count == 0)

    }
}

fn main(){
    let s = String::from("anagram");
    let t = String::from("nagaram");

    println!("{}", Solution::is_anagram(s, t));

    let s = String::from("rat");
    let t = String::from("car");

    println!("{}", Solution::is_anagram(s, t));
}