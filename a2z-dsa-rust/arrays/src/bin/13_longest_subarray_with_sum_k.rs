//using two pointer approach
fn longest_subarray_with_sum_k(nums: &[i32], k: i32) -> usize {
    let length = nums.len();
    let (mut left, mut right) = (0, 0);
    let mut current_sum = nums[0];
    let mut max_length = 0;

    while right < length {
        if current_sum == k {
            max_length = max_length.max(right - left + 1);
        }

        if current_sum < k {
            right += 1;
            if right < length {
                current_sum += nums[right];
            }
        } else {
            current_sum -= nums[left];
            left += 1;
        }
    }
    max_length
    //Time complexity is O(n) since we traverse the array at most twice.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}

fn main() {
    let nums = vec![2, 3, 5, 1, 9];
    let k = 10;
    let length = longest_subarray_with_sum_k(&nums, k);
    println!("The length of the longest subarray with sum {} is: {}", k, length);
}