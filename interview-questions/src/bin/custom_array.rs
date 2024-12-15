/*
Implement a custom array in Rust with error handling using Result?
*/

struct MyArray<T> {
    data:Vec<T>,
}

impl<T> MyArray<T> {
    // Constructor to create a new custom array
    fn new() -> Self {
        MyArray {data : Vec::new()}
    }

    //Add an element to the array
    fn push(&mut self, value: T){
        self.data.push(value);
    }

    //Get an element by index, returning Result for error handling
    fn get(&self, index: usize) -> Result<&T, String> {
        self.data.get(index).ok_or_else(|| format!("Index {} out of bounds", index))
    }

    //Set an element at the specified index, returning Result for error handling
    fn set(&mut self, index: usize, value: T) -> Result<(), String> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

    //Get the length of the array
    fn len(&self) -> usize {
        self.data.len()
    }
}
fn main() {
    let mut my_array = MyArray::new();

    my_array.push(1);
    my_array.push(2);
    my_array.push(3);

    // Get an element safely using Result
    match my_array.get(1) {
        Ok(value) => println!("Element at index 1: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // Try setting a value at a valid index
    match my_array.set(1, 99) {
        Ok(()) => println!("Value at index 1 updated successfully"),
        Err(err) => println!("Error: {}", err),
    }

    // Try setting a value at an invalid index
    match my_array.set(5, 100) {
        Ok(()) => println!("Value at index 5 updated successfully"),
        Err(err) => println!("Error: {}", err),
    }

    // Get an element at an invalid index
    match my_array.get(5) {
        Ok(value) => println!("Element at index 5: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    println!("Array length: {}", my_array.len());

}