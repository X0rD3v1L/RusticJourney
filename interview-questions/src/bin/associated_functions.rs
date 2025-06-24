struct Circle {
    radius: f64,
}

impl Circle {
    // Associated function
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method (not associated function)
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let c = Circle::new(5.0);  // calling associated function
    println!("Area: {}", c.area());
}