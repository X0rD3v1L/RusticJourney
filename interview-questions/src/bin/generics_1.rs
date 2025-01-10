fn add_numbers<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    // let sum = a + b; // `a` and `b` are still usable after this due to `Copy`
    // println!("Values after addition: a = {:?}, b = {:?}", a, b); // Reusing `a` and `b`
    // sum
    a + b
}

fn main() {
    let int_num1: u64 = 10;
    let int_num2: u64 = 20;
    let result_int = add_numbers(int_num1, int_num2);
    println!("The sum of u64 is: {}", result_int);

    let float_num1: f64 = 10.5;
    let float_num2: f64 = 20.75;
    let result_float = add_numbers(float_num1, float_num2);
    println!("The sum of f64 is: {}", result_float);
}
