fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}
fn get_value(val: Option<i32>) -> i32 {
    match val {
        Some(x) => x,
        None => 0,
    }
}
fn main() {
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    let val = Some(10);
    println!("Value: {}", get_value(val));
}