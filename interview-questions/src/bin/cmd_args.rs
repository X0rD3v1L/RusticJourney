use std::env;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are provided
    if args.len() < 2 {
        println!("Usage: {} <name> [age]", args[0]);
        return;
    }

    // Get the name argument
    let name = &args[1];

    // Check for the optional age argument
    if args.len() > 2 {
        match args[2].parse::<u8>() {
            Ok(age) => println!("Hello, {}! You are {} years old.", name, age),
            Err(_) => println!("Error: Age must be a positive number."),
        }
    } else {
        println!("Hello, {}! Your age is not specified.", name);
    }
}
