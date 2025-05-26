fn count_sub(s: &str) -> usize {
    let n = s.len();
    let chars: Vec<char> = s.chars().collect();

    // Count of substrings with all unique characters
    let mut ans = 0;

    // Frequency array for 26 lowercase English letters
    let mut cnt = vec![0; 26];

    let (mut i, mut j) = (0, 0);

    while i < n {
        if j < n && cnt[(chars[j] as u8 - b'a') as usize] == 0 {
            cnt[(chars[j] as u8 - b'a') as usize] += 1;

            // All substrings ending at j and starting from i to j
            ans += j - i + 1;

            j += 1;
        } else {
            cnt[(chars[i] as u8 - b'a') as usize] -= 1;
            i += 1;
        }
    }

    ans
}

fn main() {
    let s = "gffg"; //( "g", "gf", "f", "f", "fg", "g" ) 
    println!("{}", count_sub(s));
}

//https://www.geeksforgeeks.org/count-of-substrings-having-all-distinct-characters/