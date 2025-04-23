struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freq = [0; 26];

        for char in s.chars() {
            freq[(char as u8 - b'a') as usize] += 1;
        }

        for (i,c) in s.chars().enumerate() {
            if freq[(c as u8 - b'a') as usize] == 1 {
                return i as i32
            } 
        }

        -1
    }
}

fn main() {
    let s = String::from("loveleetcode");

    println!("{}", Solution::first_uniq_char(s));
}