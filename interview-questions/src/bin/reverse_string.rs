fn main() {
    let name = "ben";

    let mut rev = String::new();

    for i in (0..name.len()).rev() {
        rev.push(name.chars().nth(i).unwrap());
    }

    println!("Original : {}", name);
    println!("Reversed : {}", rev);
}

/*
fn reverse_string(name: &str) -> String {
    let mut chars = name.as_bytes().to_vec(); // Convert to a mutable byte vector
    let mut start = 0;
    let mut end = chars.len() - 1;

    while start < end {
        chars.swap(start, end); // Swap bytes at start and end
        start += 1;
        end -= 1;
    }

    // Convert bytes back to a string
    String::from_utf8(chars).unwrap()
}
*/

/*
fn reverse_string(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    for i in (0..bytes.len()).rev() {
        result.push(bytes[i] as char);
    }
    result
}
*/

/*
fn reverse_string(s: String) -> String {
    let mut reversed = String::new();
    
    for ch in s.chars().rev() {
        reversed.push(ch);
    }
    
    reversed
}

fn main() {
    let s = String::from("hello");
    println!("{}", reverse_string(s));

    let s2 = String::from("hé🙂");
    println!("{}", reverse_string(s2));
}
*/