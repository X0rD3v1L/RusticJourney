use std::collections::HashSet;
use std::usize;

fn first_repeating_element(arr: &[i32]) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut min_index = usize::MAX;

    for i in (0..arr.len()).rev() {
        if seen.contains(&arr[i]) {
            min_index = min_index.min(i);
        } else {
            seen.insert(arr[i]);
        }
    }

    if min_index == usize::MAX {
        None
    } else {
        Some(min_index)
    }
}

fn main() {
    let arr = [10, 5, 3, 4, 3, 5, 6];
    match first_repeating_element(&arr) {
        Some(index) => println!("First repeating element is {} at {}", arr[index], index),
        None => println!("No repeating found!"),
    }
}
