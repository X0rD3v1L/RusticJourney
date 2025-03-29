use std::ops::Add;
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point; // Specifies the return type
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2; // Uses overloaded `+`
    println!("Point({}, {})", p3.x, p3.y);
}

// use std::ops::Mul;

// struct Square {
//     side: u32,
// }

// impl Mul for Square {
//     type Output = u32;
    
//     fn mul(self, other: Square) -> u32 {
//         self.side * other.side
//     }
// }

// fn main() {
//     let s1 = Square { side: 4 };
//     let s2 = Square { side: 5 };

//     let area = s1 * s2; // Uses overloaded `*`
//     println!("Area: {}", area); // Output: Area: 20
// }