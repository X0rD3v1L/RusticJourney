fn move_zeroes_to_end(arr: &mut Vec<i32>) {
    let mut last_non_zero_index = 0;
    let mut temp;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            temp = arr[last_non_zero_index];
            arr[last_non_zero_index] = arr[i];
            arr[i] = temp;
            last_non_zero_index += 1;
        }
    }
}   

fn main(){
    let mut arr = vec![0, 1, 0, 3, 12];
    println!("Original array: {:?}", arr);
    move_zeroes_to_end(&mut arr);
    println!("Array after moving zeroes to the end: {:?}", arr);
}