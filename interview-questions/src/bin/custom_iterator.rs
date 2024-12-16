/*
Implement a custom iterator in Rust
*/
struct Counter {
    count : u32,
}

impl Counter {
    fn new() -> Self {
        Counter {count : 0}
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else {
            None
        }
    }
}
fn main() {
    let mut counter = Counter::new();
    while let Some(x) = counter.next() {
        println!("{}", x);
    }
}