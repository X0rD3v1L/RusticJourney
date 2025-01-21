/*
Copy trait is designed for types that have a fixed, small size and can be duplicated by simply copying bits.
Examples of such types are primitive types like i32, f32, or arrays like [i32; 4].
These are stored entirely on the stack.

Rust provides the Clone trait for explicitly creating deep copies of heap-allocated data like String.
This avoids confusion and ensures that copying is an explicit action.
*/
fn main() {
    // Ownership with numbers (primitive types)
    let x = 5;  // `x` owns the value 5
    let y = x;  // `y` is a copy of `x`

    // Both `x` and `y` are valid because integers implement the Copy trait
    println!("x: {}, y: {}", x, y); // Both can be used without issues

    // Ownership with heap-allocated numbers (Boxed integers)
    let a = Box::new(10);  // `a` owns the value 10, stored on the heap
    let b = a;             // Ownership is transferred to `b`

    // ❌ `a` can no longer be used because ownership is moved
    // println!("a: {}", a); // This would cause a compile-time error
    println!("b: {}", b); // `b` now owns the value

    // Borrowing the value instead of transferring ownership
    let c = Box::new(20);   // `c` owns the value 20
    let borrowed = &c;      // Borrow the value of `c`
    println!("c: {}, borrowed: {}", c, borrowed);

    // Mutable borrowing
    let mut d = Box::new(30); // `d` owns a mutable heap-allocated integer
    modify_value(&mut d);     // Borrow `d` mutably to modify it
    println!("Modified d: {}", d);
}

fn modify_value(num: &mut Box<i32>) {
    **num += 10; // Dereference twice to modify the value inside the Box
}
