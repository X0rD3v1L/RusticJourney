fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();

    for i in 0..len {
        let mut swapped = false;

        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j,  j + 1);
                swapped = true;
            }
        }

        //if no elements are swapped the array is already sorted
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("Sorted array: {:?}", numbers); 
}