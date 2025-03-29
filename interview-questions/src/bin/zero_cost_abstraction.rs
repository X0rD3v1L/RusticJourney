/*
Zero-cost abstractions mean that the abstractions provided by Rust (like iterators, closures) 
do not incur runtime overhead compared to hand-written equivalents.
*/
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled); // [2, 4, 6, 8]
}