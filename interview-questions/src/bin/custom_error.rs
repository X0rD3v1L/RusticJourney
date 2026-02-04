#[derive(Debug)]
enum MathError {
    DivideByZero,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivideByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10, 0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}
