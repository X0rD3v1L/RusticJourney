use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_k_messed(arr: &mut Vec<i32>, k: usize) {
    let mut heap = BinaryHeap::new();
    let n = arr.len();
    let mut index = 0;

    for i in 0..std::cmp::min(k + 1, n) {
        heap.push(Reverse(arr[i]));
    }

    for i in k + 1..n {
        arr[index] = heap.pop().unwrap().0;
        index += 1;
        heap.push(Reverse(arr[i]));
    }

    while let Some(Reverse(val)) = heap.pop() {
        arr[index] = val;
        index += 1;
    }
}

fn main() {
    let mut arr = vec![6, 5, 3, 2, 8, 10, 9];
    let k = 3;
    sort_k_messed(&mut arr, k);
    println!("{:?}", arr);
}