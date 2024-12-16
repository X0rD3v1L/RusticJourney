/*
Unsafe rust example

Unsafe Rust allows developers to perform low-level operations that bypass Rust's usual safety guarantees. 
This includes operations like dereferencing raw pointers, calling unsafe functions, or accessing mutable static variables. 
Unsafe Rust is necessary in scenarios where performance and control are critical, such as interacting with hardware,
building abstractions like custom allocators, or interfacing with C code via FFI (Foreign Function Interface).

*/

fn main() {
    let value: i32 = 42;

    //Create raw pointers
    let raw_pointer = &value as *const i32;

    unsafe {
        //Dereference the raw pointer
        println!("Value at raw pointer: {}", *raw_pointer);
    }
    /*
    Raw pointers (*const T or *mut T) bypass Rust's safety checks.
    Dereferencing a raw pointer is unsafe because it can lead to undefined behavior,
    if the pointer is invalid.
    */
}

/*
static mut COUNTER: i32 = 0;

fn increment(){
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    increment();
    increment();

    unsafe{
        println!("COUNTER: {}", COUNTER);
    }
}

// Rust can't guarantee thread-safety for mutable static variables.
// The programmer must ensure no data races occur when accessing the variable.

*/