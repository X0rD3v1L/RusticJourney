struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like a static method)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method (takes &self as the first parameter)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
}
