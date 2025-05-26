use std::collections::HashSet;

fn distinct_substrings(s: &str) -> usize {
    let n = s.len();
    let mut substrings = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    for i in 0..n {
        let mut freq = vec![false; 26];
        let mut substring = String::new();

        for j in i..n {
            let pos = (chars[j] as u8 - b'a') as usize;

            // If character is repeated, break the inner loop
            if freq[pos] {
                break;
            }

            // Mark character as seen
            freq[pos] = true;

            // Add current character to substring
            substring.push(chars[j]);

            // Insert substring into HashSet
            substrings.insert(substring.clone());
        }
    }
    println!("{:?}", substrings);
    substrings.len()
}

fn main() {
    let s = "gffg";
    println!("{}", distinct_substrings(s)); //{"f", "gf", "g", "fg"}
}
