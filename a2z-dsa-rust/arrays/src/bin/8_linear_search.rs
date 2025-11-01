fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}
fn main() {
    let arr = vec![10, 23, 45, 70, 11, 15];
    let target = 70;
    match linear_search(&arr, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }
}