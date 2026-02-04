fn two_sum(arr: &Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let current_sum = arr[left] + arr[right];
        if current_sum == target {
            return Some((left, right));
        } else if current_sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
} 
 
fn main(){
    let arr = vec![2, 7, 11, 15];
    let target = 9;
    let ans = two_sum(&arr, target);   
    match ans {
        Some((i, j)) => println!("Indices of elements that sum to {} are: {} and {}", target, i, j),
        None => println!("No two elements sum to {}", target),
    }
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space 
}