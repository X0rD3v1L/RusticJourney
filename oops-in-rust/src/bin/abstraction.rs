trait Summable<T> {
    fn sum(&self, other: T) -> T;
}

impl Summable<i32> for i32 {
    fn sum(&self, other: i32) -> i32 {
        self + other
    }
}

fn main() {
    let x = 5;
    let y = 10;
    println!("Sum: {}", x.sum(y));
}

/*
Rust traits support generic functions, allowing flexible implementations.

Abstraction in programming allows you to define general behavior without specifying the exact details.
In Rust, this is often achieved using generic functions and trait bounds.
*/