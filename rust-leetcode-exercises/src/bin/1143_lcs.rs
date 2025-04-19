use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();

        for i in 1..=n {
            for j in 1..=m {
                let (i_usize, j_usize) = (i as usize, j as usize);
                if t1[i_usize - 1] == t2[j_usize - 1] {
                    dp[i_usize][j_usize] = 1 + dp[i_usize - 1][j_usize - 1];
                } else {
                    dp[i_usize][j_usize] = max(dp[i_usize - 1][j_usize], dp[i_usize][j_usize - 1]);
                }
            }
        }
        
        let mut i = n;
        let mut j = m;
        let mut lcs = Vec::new();

        while i > 0 && j > 0 {
            if t1[i - 1] == t2[j - 1]{
                lcs.push(t1[i-1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }

        lcs.reverse();
        println!("{}", String::from_utf8(lcs.clone()).unwrap());

        dp[n][m]

        
    }
}

fn main() {
    let text1 = String::from("abcde");
    let text2 = String::from("ace");

    println!("{}", Solution::longest_common_subsequence(text1, text2));
}