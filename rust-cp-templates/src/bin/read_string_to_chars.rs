use std::io;

fn take_string() -> Vec<char> {
    // Initiate input variable
    let mut input = String::new();

    // Read into input
    io::stdin().read_line(&mut input).unwrap();

    // Convert input to vector of characters
    let vec:Vec<char> = input.trim().chars().collect();

    // Return vector
    return vec;
}

// Driver code
fn main() {
    let str1 = take_string();
    println!("{:?}", str1);
    println!("First and last characters : {} {}", str1[0], str1[str1.len()-1]);
}