fn find_repeating(nums: &mut Vec<i32>) {
    nums.sort_unstable();
    
    let n = nums.len();
    let mut i = 0;
    
    while i <  n {
        let first_index = i;
        while i < n - 1 && nums[i] == nums[i + 1] {
            i += 1;
        }
        let last_index = i;
        
        let freq = (last_index - first_index) + 1;
        
        if freq >= 2 {
            println!("{}", nums[i]);
        }
        
        i += 1;
    }
}

fn main() {
    let mut arr = vec![4, 3, 2, 7, 8, 2, 3, 1];
    
    find_repeating(&mut arr);
    
}