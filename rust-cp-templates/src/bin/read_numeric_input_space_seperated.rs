use std::io;

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    return arr;
}


fn main(){
    let arr = take_vector();
    let mut sum = 0;
    for i in 0..arr.len(){
        sum+=arr[i];
    }
    println!("{}", sum);
}

/*
use std::io;
use std::str::FromStr;

//FromStr trait - Allows us to convert from a string to a type (like isize or usize).

//The where T::Err: std::fmt::Debug bound ensures that we can print the error if parsing fails.

fn take_vector<T: FromStr>() -> Vec<T> 
where 
    T::Err: std::fmt::Debug,  // Ensure parse errors can be displayed
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect()
}

fn main() {
    // Test with isize (signed integers)
    println!("Enter signed integers:");
    let arr: Vec<isize> = take_vector();
    let sum: isize = arr.iter().sum();
    println!("Sum of signed integers: {}", sum);

    // Test with usize (unsigned integers)
    println!("\nEnter unsigned integers:");
    let arr: Vec<usize> = take_vector();
    let sum: usize = arr.iter().sum();
    println!("Sum of unsigned integers: {}", sum);
}
*/