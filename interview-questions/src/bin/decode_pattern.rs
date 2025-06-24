/*
Given the encoded string, str = "1226#24#(2)" = "abzxx"
here a is denoted as 1, b as 2 .... and i as 9
j is denoted as 10#, k as 11# and so on
If the character occurs more than 1 times the freq is added next to encoded character in the paranthesis.
Find the frequency of all the characters found in the string.
o/p -> [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1]
*/

fn decode_frequencies(s: &str) -> Vec<usize> {
    let mut freq = vec![0; 26];
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 1;
        let ch;

        if i + 2 < chars.len() && chars[i + 2] == '#' {
            // Handle encoded characters like "10#"
            let num: usize = format!("{}{}", chars[i], chars[i + 1])
                .parse()
                .unwrap();
            println!("num: {}", num);
            ch = (b'a' + (num - 1) as u8) as char;
            i += 3;

            // Check for frequency e.g. "(2)"
            if i < chars.len() && chars[i] == '(' {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != ')' {
                    j += 1;
                }
                count = s[i + 1..j].parse::<usize>().unwrap();
                i = j + 1;
            }
        } else {
            // Single-digit character (1 to 9)
            let num = chars[i].to_digit(10).unwrap() as usize;
            ch = (b'a' + (num - 1) as u8) as char;
            i += 1;
        }

        freq[(ch as u8 - b'a') as usize] += count;
    }

    freq
}

fn main() {
    let input = "1226#24#(2)";
    let result = decode_frequencies(input);
    println!("{:?}", result);
}
