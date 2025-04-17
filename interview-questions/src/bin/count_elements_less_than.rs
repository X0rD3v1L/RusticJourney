/*
For each element in arr2, count how many elements in arr1 are strictly less than it.
*/
fn count_elements_less(arr1: &[usize], arr2: &[usize]) {
    const MAX: usize = 100010;
    let mut freq = vec![0; MAX];
    let mut prefix = vec![0; MAX];

    for &num in arr1 {
        freq[num] += 1;
    }

    prefix[0] = freq[0];
    for i in 1..MAX {
        prefix[i] = prefix[i - 1] + freq[i];
    }

    for &num in arr2 {
        if num == 0 {
            print!("0 ");
        } else {
            print!("{} ", prefix[num - 1]);
        }
    }
    println!();
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 7, 9];
    let arr2 = vec![0, 1, 2, 1, 1, 4];

    count_elements_less(&arr1, &arr2);
}
/*
fn binary_search_strictly_less(arr: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // `left` is the count of elements strictly less than target
    left
}

fn count_elements_less(arr1: &[usize], arr2: &[usize]) {
    let mut sorted_arr1 = arr1.to_vec();
    sorted_arr1.sort_unstable(); 

    for &elem in arr2 {
        let count = binary_search_strictly_less(&sorted_arr1, elem);
        print!("{} ", count);
    }

    println!();
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 7, 9];
    let arr2 = vec![0, 1, 2, 1, 1, 4];

    count_elements_less(&arr1, &arr2); // Output: 0 0 1 0 0 3
}
*/