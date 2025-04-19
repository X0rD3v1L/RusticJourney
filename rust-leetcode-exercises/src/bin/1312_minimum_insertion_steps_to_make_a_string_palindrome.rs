use std::cmp::max;

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        s.len() as i32 - Solution::longest_palindrome_subseq(s)
    }
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        let t1 = s.as_bytes();
        let text2: String = s.chars().rev().collect();
        let t2 = text2.as_bytes();

        for i in 1..=n {
            for j in 1..=n {
                let (i_usize, j_usize) = (i as usize, j as usize);
                if t1[i_usize - 1] == t2[j_usize - 1] {
                    dp[i_usize][j_usize] = 1 + dp[i_usize - 1][j_usize - 1];
                } else {
                    dp[i_usize][j_usize] = max(dp[i_usize - 1][j_usize], dp[i_usize][j_usize - 1]);
                }
            }
        }
        
        let mut i = n;
        let mut j = n;
        let mut lps = Vec::new();

        while i > 0 && j > 0 {
            if t1[i - 1] == t2[j - 1]{
                lps.push(t1[i-1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }

        lps.reverse();        
        let lps_str = String::from_utf8(lps.clone().into_iter().rev().collect()).unwrap();
        println!("Longest Palindromic Subsequence: {}", lps_str);

        // To find insertions needed (characters not in LPS)
        let mut insertions = Vec::new();
        let s_chars: Vec<char> = s.chars().collect();
        let lps_chars: Vec<char> = lps.iter().rev().map(|c| *c as char).collect();
        let mut lps_idx = 0;

        for ch in s_chars.iter() {
            if lps_idx < lps_chars.len() && *ch == lps_chars[lps_idx] {
                lps_idx += 1;
            } else {
                insertions.push(*ch);
            }
        }

        println!("{:?}", insertions);

        dp[n][n]
    }
}

fn main() {
    let s = String::from("mbadm");
    println!("{}", Solution::min_insertions(s));
}