fn check_if_array_is_sorted(arr: &[i32]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
    // Time complexity is O(n) since we traverse the array once.
    // Space complexity is O(1) since we use only a fixed amount of extra space
}

fn main(){
    let arr = vec![1, 2, 3, 4, 5];
    let check_sorted = check_if_array_is_sorted(&arr);
    if check_sorted { 
        println!("The array is sorted.");
    } else {
        println!("The array is not sorted.");
    }
}