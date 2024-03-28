// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
// * Use a function to display the result
fn display(result: i32){
    // * Use the "{:?}" token in the println macro to display the result
    println!("{:?} ",result);
}

fn main() {
    display(add(1,2));
}