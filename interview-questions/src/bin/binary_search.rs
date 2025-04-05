fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2; // Prevents overflow

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None // Target not found
}

fn main() {
    let arr = vec![0, 1, 2, 3, 4, 5];
    match binary_search(&arr, 2) {
        Some(index) => println!("Element 2 found at index {}", index),
        None => println!("Element 2 not found"),
    }
}
