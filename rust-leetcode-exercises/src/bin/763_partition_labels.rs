struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = Vec::new();

        let mut last_occurence = vec![0; 26];

        for (index, &char) in chars.iter().enumerate() {
            last_occurence[(char as u8 - b'a') as usize] = index;
        }

        let mut start = 0;
        let mut end = 0;

        for (index, &char) in chars.iter().enumerate() {
            end = end.max(last_occurence[(char as u8 - b'a' as u8) as usize]);
            if index == end {
                ans.push((end - start + 1) as i32);
                start = index + 1;
            }
        }

        ans
    }
}

fn main() {
    let s = String::from("ababcc");
    println!("{:?}", Solution::partition_labels(s));
}