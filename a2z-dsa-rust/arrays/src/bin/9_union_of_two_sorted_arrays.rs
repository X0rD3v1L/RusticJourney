fn union_of_two_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut union = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            if union.last() != Some(&arr1[i]) {
                union.push(arr1[i]);
            }
            i += 1;
        } else if arr1[i] > arr2[j] {
            if union.last() != Some(&arr2[j]) {
                union.push(arr2[j]);
            }
            j += 1;
        } else {
            if union.last() != Some(&arr1[i]) {
                union.push(arr1[i]);
            }
            i += 1;
            j += 1;
        }
    }
    while i < arr1.len() {
        if union.last() != Some(&arr1[i]) {
            union.push(arr1[i]);
        }
        i += 1;
    }
    while j < arr2.len() {
        if union.last() != Some(&arr2[j]) {
            union.push(arr2[j]);
        }
        j += 1;
    }
    union
    //Time complexity is O(m + n) where m and n are the sizes of the two arrays.
    //Space complexity is O(m + n) in the worst case when there are no common elements.
}
fn main() {
    let arr1 = vec![1, 2, 2, 3, 4];
    let arr2 = vec![2, 3, 5, 6];
    let union = union_of_two_sorted_arrays(&arr1, &arr2);
    println!("Union of the two sorted arrays: {:?}", union);
}