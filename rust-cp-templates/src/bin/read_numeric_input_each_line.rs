use std::io;

fn take_int() -> i32 {
    // Declare string
    let mut input = String::new();

    // Input string
    io::stdin().read_line(&mut input).unwrap();

    // Return number
    return input.trim().parse().unwrap();
}

pub fn main() {
    let number1 = take_int();
    let number2 = take_int();
    println!("{}", number1 * number2);
}