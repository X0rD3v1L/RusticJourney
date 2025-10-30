//Left Roate by one place
fn left_rotate_by_one(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let first_element = arr[0];
    let n = arr.len();
    for i in 0..n - 1 {
        arr[i] = arr[i + 1];
    }
    arr[n - 1] = first_element;
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}
fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    println!("Original array: {:?}", arr);
    left_rotate_by_one(&mut arr);
    println!("Array after left rotation by one place: {:?}", arr);
}