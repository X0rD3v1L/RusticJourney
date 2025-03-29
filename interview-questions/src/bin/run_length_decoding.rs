fn run_length_decoding(encoded: String) -> String {
    let mut decoded = String::new();
    let chars: Vec<char> = encoded.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 0;

        while i < chars.len() && chars[i].is_numeric() {
            count = count * 10 + chars[i].to_digit(10).unwrap();
            i += 1;
        }

        if i < chars.len() {
            decoded.push_str(&chars[i].to_string().repeat(count as usize));
            i += 1;
        }
    }

    decoded
}

fn main() {
    let encoded = String::from("1B1e1n1a1r1j2e");
    let decoded = run_length_decoding(encoded);
    println!("Decoded string: {}", decoded);
}
