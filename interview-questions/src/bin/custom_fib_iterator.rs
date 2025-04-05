struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.a;
        self.a = self.b;
        self.b = next_value + self.b;
        Some(next_value)
    }
}

fn main() {
    let mut fib = Fibonacci::new();
    
    for _ in 0..10 {
        print!("{} ", fib.next().unwrap());
    }
}
