fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[low];
    let mut i = low;
    let mut j = high;
    
    while i < j {
        while i < high && arr[i] <= pivot {
            i += 1;
        }
        while j > low && arr[j] > pivot {
            j -= 1;
        }
        if i < j {
            arr.swap(i, j);
        }
    }
    arr.swap(low, j);
    j
}

fn qs(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let p_index = partition(arr, low, high);
        if p_index > 0 {
            qs(arr, low, p_index - 1);
        }
        qs(arr, p_index + 1, high);
    }
}

fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }
    let len = arr.len();
    qs(&mut arr, 0, len - 1);
    arr
}

fn main() {
    let mut arr = vec![4, 6, 2, 5, 7, 9, 1, 3];
    println!("Before :: {:?}", arr);
    arr = quick_sort(arr);
    println!("After :: {:?}", arr);
}
