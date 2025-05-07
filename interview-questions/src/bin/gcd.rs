fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a = 48;
    let b = 18;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
