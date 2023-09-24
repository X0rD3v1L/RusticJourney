use std::io::{self, Write};

fn main() {
    let mut fahrenheit = String::new();
    print!("Enter Fahrenheit to convert into Celsius :: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Flush the output buffer

    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return; // Exit the program if input is not a valid number.
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{} Fahrenheit is equal to {} Celsius", fahrenheit, celsius);
}