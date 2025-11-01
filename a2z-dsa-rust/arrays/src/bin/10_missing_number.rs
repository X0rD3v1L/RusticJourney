fn find_missing_number(arr: &[i32], n: i32) -> i32 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = arr.iter().sum();
    expected_sum - actual_sum
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}
fn main() {
    let arr = vec![0, 1, 2, 4, 5];
    let n = 5; // Since numbers are from 0 to 5, n is 5
    let missing_number = find_missing_number(&arr, n);
    println!("The missing number is: {}", missing_number);
}