trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area : {:.2} ", shape.area());
}

fn main() {
    let circle = Circle { radius : 2.0 };
    let square = Square { side : 3.0 };

    print_area(&circle);
    print_area(&square);
    
}