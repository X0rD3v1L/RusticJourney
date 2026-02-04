fn sort_012(arr: &mut [i32]) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = arr.len() - 1;

    while mid <= high {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                arr.swap(mid, high);
                if high == 0 { break; } // Prevent underflow
                high -= 1;
            }
            _ => panic!("Array can only contain 0s, 1s, and 2s"),
        }
    }
}
fn main() {
    let mut arr = vec![0, 1, 2, 0, 1, 2, 1, 0];
    sort_012(&mut arr);
    println!("Sorted array: {:?}", arr);
}