use std::cell::RefCell;

struct MyStruct {
    value: RefCell<i32>,
}

impl MyStruct {
    fn new(value: i32) -> Self {
        MyStruct{
            value: RefCell::new(value),
        }
    }
    
    fn increment(&self) {
        let mut val = self.value.borrow_mut();
        *val += 1;
    }
    
    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

fn main() {
    let my_struct = MyStruct::new(5);
    println!("Initial value: {}", my_struct.get_value()); // Prints 5

    // The increment function mutably borrows the value inside the RefCell
    my_struct.increment();

    println!("Value after increment: {}", my_struct.get_value()); // Prints 6
}