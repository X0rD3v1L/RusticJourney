fn find_median(mut arr: Vec<i32>) -> f64 {
    arr.sort();
    let len = arr.len();
    let mid = len / 2;
    
    if len % 2 == 1 {
        arr[mid] as f64
    } else {
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    }
}

fn main() {
    let arr = vec![2, 3, 4, 5, 1, 2, 3];
    let median = find_median(arr);
    println!("Median: {}", median);
}
