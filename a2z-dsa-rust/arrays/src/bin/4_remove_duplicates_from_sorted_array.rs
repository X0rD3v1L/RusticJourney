fn remove_duplicates_from_sorted_array(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut unique_index = 0;

    for i in 1..arr.len() {
        if arr[i] != arr[unique_index] {
            unique_index += 1;
            arr[unique_index] = arr[i];
        }
    }

    println!("Unique elements: {:?}", &arr[..(unique_index + 1)]);
    (unique_index + 1) as usize
    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}


fn main(){
    let mut arr = vec![1, 2, 2, 3, 4, 4, 5];
    let unique_count = remove_duplicates_from_sorted_array(&mut arr);
    println!("{:?}", arr);
    println!("Number of unique elements: {}", unique_count);
}