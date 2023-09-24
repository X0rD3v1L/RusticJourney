use std::io::{self, Write};

fn main(){
    let mut n = String::new();

    print!("Enter n to find the nth Fibonacci number :: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Flush the output buffer

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return; // Exit the program if input is not a valid number.
        }
    };

    println!("nth Fibonacci number is {}", nth_fib(n));

}

fn nth_fib(n : u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _i in 1..n{
        let tmp = a + b;
        a = b;
        b = tmp;
    }

    return a;
}