//left rotate by k places
fn left_rotate_by_k_places(arr: &mut [i32], k: usize) {
    let n = arr.len();
    if n == 0 {
        return;
    }
    let k = k % n; // In case k is greater than n
    reverse(arr, 0, n - 1);
    reverse(arr, 0, k - 1);
    reverse(arr, k, n - 1);
    //Time complexity is O(n) since we traverse the array a constant number of times.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}
fn reverse(arr: &mut [i32], start: usize, end: usize) {
    let mut left = start;
    let mut right = end;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}
fn main() {
    let mut arr = vec![1,2,3,4,5,6,7];
    let k = 3;
    println!("Original array: {:?}", arr);
    left_rotate_by_k_places(&mut arr, k);
    println!("Array after left rotation by {} places: {:?}", k, arr);
}