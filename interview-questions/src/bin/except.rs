fn main() {
    let result: Result<i32, &str> = Ok(42);

    // Works fine
    println!("The result is: {}", result.expect("Failed to get result"));

    let error_result: Result<i32, &str> = Err("Something went wrong");

    // Panics here with custom message:
    // thread 'main' panicked at 'Oops: Something went wrong'
    println!("This will panic: {}", error_result.expect("Oops"));
}

//Safe for prototyping but avoid in production