fn max_sub_array(nums: &[i32]) -> i32 {
    let mut max_sum = i32::MIN;
    let mut sum = 0;

    for &elem in nums {
        sum += &elem;

        max_sum = std::cmp::max(max_sum, sum);

        if sum < 0 {
            sum = 0;
        }
    }
    max_sum
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let ans = max_sub_array(&nums);
    println!("The maximum subarray sum is: {}", ans);
}