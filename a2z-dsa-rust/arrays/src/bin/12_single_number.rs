fn single_number(arr: &[i32]) -> i32 {
    let mut count = 0;
    let mut res = 0;
    while count < arr.len(){
        res ^= arr[count];
        count += 1;
    }

    return res;
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}
fn main() {
    let arr = vec![4, 1, 2, 1, 2];
    let unique_number = single_number(&arr);
    println!("The single number is: {}", unique_number);
}