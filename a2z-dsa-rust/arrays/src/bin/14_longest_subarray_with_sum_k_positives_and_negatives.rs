fn longest_subarray_with_sum_k(arr: &Vec<i32>, k: i32) -> i32 {
    let mut sum_index_map = std::collections::HashMap::new();
    let mut current_sum = 0;
    let mut max_length = 0;

    for (i, &value) in arr.iter().enumerate() {
        current_sum += value;

        if current_sum == k {
            max_length = i as i32 + 1;
        }

        if let Some(&start_index) = sum_index_map.get(&(current_sum - k)) {
            if (i as i32 - start_index) > max_length {
                max_length = i as i32 - start_index;
            }
        }

        sum_index_map.entry(current_sum).or_insert(i as i32);
    }
    max_length
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(n) since we use a hashmap to store sums.
}
fn main(){
    let arr = vec![4, -1, -3, 4, 2, -3, 1, 1, -1];
    let k = 3;
    let length = longest_subarray_with_sum_k(&arr, k);
    println!("The length of the longest subarray with sum {} is: {}", k, length);
}