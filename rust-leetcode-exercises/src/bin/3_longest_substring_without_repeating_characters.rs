struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut hash_map: [i32;256] = [-1;256];
        let (mut l, mut r, mut max_len) = (0, 0, 0);

        while r < n {
            let ch = s[r] as usize;
            if hash_map[ch] != -1 && hash_map[ch] >= l as i32 {
                l = (hash_map[ch] + 1) as usize;
            }

            hash_map[ch] = r as i32;
            let len = r - l + 1;
            max_len = max_len.max(len);
            r += 1;
        }

        max_len as i32
    }
}

fn main() {
    let s = String::from("abcabcbb");
    println!("{}", Solution::length_of_longest_substring(s));
}