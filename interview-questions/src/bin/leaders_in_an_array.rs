fn find_leaders(nums: &Vec<i32>) -> Vec<i32> {
    let mut leaders: Vec<i32> = Vec::new();
    let n = nums.len();
    let mut max_right = nums[n - 1];
    leaders.push(max_right);

    for index in (0..=n-2).rev() {
        if nums[index] >= max_right {
            max_right = nums[index];
            leaders.push(nums[index]);
        }
    }
    
    leaders.reverse();
    leaders
}


fn main() {
    let nums = vec![16, 17, 4, 3, 5, 2];
    let leaders = find_leaders(&nums);
    println!("{:?}", leaders);
}