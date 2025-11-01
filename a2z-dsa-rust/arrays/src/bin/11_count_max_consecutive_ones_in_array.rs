fn count_max_consecutive_ones(arr: &[i32]) -> i32 {
    let mut max_count = 0;
    let mut current_count = 0;

    for &item in arr.iter() {
        if item == 1 {
            current_count += 1;
            if current_count > max_count {
                max_count = current_count;
            }
        } else {
            current_count = 0;
        }
    }

    max_count
    // Time complexity is O(n) since we traverse the array once.
    // Space complexity is O(1) since we use only a fixed amount of extra space
}
fn main() {
    let arr = vec![1, 1, 0, 1, 1, 1];
    let max_consecutive_ones = count_max_consecutive_ones(&arr);
    println!("The maximum consecutive ones are: {}", max_consecutive_ones);
}