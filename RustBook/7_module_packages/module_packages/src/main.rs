use std::io;
mod my_module;
use my_module::say_hello;

fn main() {
    println!("Hello, world!");

    say_hello();

    my_module::say_goodbye();

    let mut input = String::new();
    println!("\nEnter your name");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Hello, {}", input);
}
