fn rearrange_array(nums: Vec<i32>) -> Vec<i32>{
    let mut res = vec![0; nums.len()];
    let (mut pos_index, mut neg_index) = (0, 1);
    for index in 0..nums.len() {
        if nums[index] < 0 {
            res[neg_index] = nums[index];
            neg_index += 2;
        } else {
            res[pos_index] = nums[index];
            pos_index += 2;
        }
    }
    res
}
//Time complexity is O(n) since we traverse the array once.
//Space complexity is O(n) since we use an array of size n.
fn main() {
    let nums = vec![3,1,-2,-5,2,-4];
    let ans = rearrange_array(nums);
    println!("{:?}", ans);
}