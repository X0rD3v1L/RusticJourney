use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Invalid integer"))
        .collect();

    // Destructure into two variables
    let (a, b) = (parts[0], parts[1]);

    println!("a = {}, b = {}", a, b);
}
