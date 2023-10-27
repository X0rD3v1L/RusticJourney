use std::collections::HashMap;

fn two_sum(nums: Vec<i32>,target: i32) -> Vec<i32> {
    let mut num_indices = HashMap::new();

    for index in 0..nums.len(){
        let complement = target - nums[index];
        if let Some(&prev_index) = num_indices.get(&complement) {
            return vec![prev_index as i32, index as i32];
        } else {
            num_indices.insert(nums[index], index);
        }
    }

    vec![]
}
fn main(){
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = two_sum(nums, target);

    
    println!("{:?}", result);
}