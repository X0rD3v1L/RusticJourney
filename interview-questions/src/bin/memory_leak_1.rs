/*
Simulate a memory leak situation in Rust
*/
use std::rc::Rc;
use std::cell::RefCell;

#[warn(dead_code)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));

    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: None,
    }));

    // Creating a reference cycle
    node1.borrow_mut().next = Some(Rc::clone(&node2));
    node2.borrow_mut().next = Some(Rc::clone(&node1));

    // Here the reference count of node1 and node2 would not drop to zero,
    // because of the circular reference, and they will never be dropped,
    // causing a memory leak.


    //Solution

    // Creating a non-circular reference
    // node1.borrow_mut().next = Some(Rc::downgrade(&node2));
    // node2.borrow_mut().next = Some(Rc::downgrade(&node1));

    // Here, the memory will be correctly freed when no longer in use
}
