struct Solution;

impl Solution {
    pub fn longest_common_substring(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();
        let mut ans = 0;
        let mut end_index = 0;

        for i in 1..=n {
            for j in 1..=m {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];

                    if dp[i][j] > ans {
                        ans = dp[i][j];
                        end_index = i;
                    }
                } else {
                    dp[i][j] = 0;
                }
            }
        }
        let lcs = if ans > 0 {
            text1[end_index - ans as usize..end_index].to_string()
        } else {
            "".to_string()
        };
        println!("{}", lcs);
        ans
    }
}

fn main() {
    let text1 = String::from("abcjklp");
    let text2 = String::from("acjkp");

    println!("{}", Solution::longest_common_substring(text1, text2));
}