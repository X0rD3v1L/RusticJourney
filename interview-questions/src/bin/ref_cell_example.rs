use std::rc::Rc;
use std::cell::RefCell;

/*
The Problem
You want two things:

Multiple parts of your code should own the counter.
Each part should be able to mutate the counter.

Rust won’t allow mutation of data behind an Rc,
because it only allows immutable access, and you’d need exclusive ownership to mutate.
*/
fn main() {
    let counter = Rc::new(RefCell::new(0)); // ✅ shared, mutable counter

    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);

    // simulate different parts of program mutating the counter
    *counter1.borrow_mut() += 1;
    *counter2.borrow_mut() += 2;

    println!("Counter value: {}", counter.borrow()); // prints 3
}

/*This is the classic Rust pattern when you need shared ownership + mutability.*/

/*
RefCell
lets you mutably borrow data even when the variable itself is immutable, but only at runtime, not compile time like &mut.
*/