fn largest_element(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    
    let mut largest = arr[0];
    for &item in arr.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    Some(largest)
    
    // Time complexity is O(n) since we traverse the array once.
    // Space complexity is O(1) since we use only a fixed amount of extra space
}

fn main() {
    let arr = vec![1, 2, 3, 4];
    let largest = largest_element(&arr);

    println!("The largest element is: {:?}", largest.unwrap_or(-1));
}
