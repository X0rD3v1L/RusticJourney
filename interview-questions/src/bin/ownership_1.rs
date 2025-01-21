fn main() {
    // Ownership: A variable owns its data
    let s1 = String::from("Hello, Rust!"); // `s1` owns the string
    println!("s1 before moving: {}", s1);

    // Move: Ownership is transferred to `s2`
    let s2 = s1; 
    // println!("{}", s1); // ❌ This will cause a compile-time error because ownership of `s1` is moved to `s2`

    println!("s2 after moving: {}", s2);

    // Clone: Explicitly copy the data to create a new ownership
    let s3 = s2.clone(); 
    println!("s2: {}, s3 (cloned): {}", s2, s3);

    // Borrowing: Using a reference to avoid ownership transfer
    let length = calculate_length(&s3); // Borrow `s3` (immutable reference)
    println!("The length of '{}' is {}.", s3, length);

    // Mutable Borrowing
    let mut s4 = String::from("Hello");
    append_world(&mut s4); // Borrow `s4` mutably
    println!("After appending: {}", s4);

    // ❌ Only one mutable borrow is allowed at a time
    // let r1 = &mut s4;
    // let r2 = &mut s4; // This would cause a compile-time error

    // ❌ Mutable and immutable borrows cannot coexist
    // let r1 = &s4;
    // let r2 = &mut s4; // This would cause a compile-time error
}

fn calculate_length(s: &String) -> usize {
    s.len() // Access the borrowed value
}

fn append_world(s: &mut String) {
    s.push_str(", world!"); // Modify the borrowed value
}
