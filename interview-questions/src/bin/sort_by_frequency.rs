use std::collections::HashMap;

fn sort_by_frequency(arr: &mut Vec<i32>) {
    // Step 1: Count frequency
    let mut freq_map: HashMap<i32, usize> = HashMap::new();
    for &num in arr.iter() {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Step 2: Sort using a stable sort
    arr.sort_by(|a, b| {
        let freq_a = freq_map[a];
        let freq_b = freq_map[b];
        freq_b.cmp(&freq_a) // Descending frequency
            .then_with(|| a.cmp(b)) // Tie-break by value (optional)
    });
}

fn main() {
    let mut arr = vec![1, 2, 3, 2, 4, 3, 1, 2];
    sort_by_frequency(&mut arr);
    println!("{:?}", arr); // Output: [2, 2, 2, 1, 1, 3, 3, 4]
}
