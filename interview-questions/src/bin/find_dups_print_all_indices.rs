use std::collections::HashMap;

fn main() {
    let input = "a a b a b c";
    
    // Split into words, map each word to chars, and flatten into a single iterator
    let chars: Vec<char> = input
        .chars()                   // Step 1: Get all characters including spaces
        .filter(|&c| !c.is_whitespace())  // Step 2: Exclude spaces
        .collect();
    

    let mut map = HashMap::new();
    
    for (index, elem) in chars.iter().enumerate() {
        map.entry(*elem).or_insert_with(Vec::new).push(index);
    }
    
    println!("{:?}", map);
}
