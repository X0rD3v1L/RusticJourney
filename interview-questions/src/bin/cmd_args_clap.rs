use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "Greet a person", long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,

    /// Age of the person (optional)
    age: Option<u8>,
}

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();

    // Print the greeting message
    match args.age {
        Some(age) => println!("Hello, {}! You are {} years old.", args.name, age),
        None => println!("Hello, {}! Your age is not specified.", args.name),
    }
}
