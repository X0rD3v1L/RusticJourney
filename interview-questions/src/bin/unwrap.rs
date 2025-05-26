fn main() {
    let some_value = Some(10);

    // Succeeds and prints the value
    println!("The value is: {}", some_value.unwrap());

    let none_value: Option<i32> = None;

    // Panics here: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
    println!("This will panic: {}", none_value.unwrap());
}

//Safe for prototyping but avoid in production