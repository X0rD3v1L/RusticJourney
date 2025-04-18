use std::cmp::min;
struct Solution;

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort_unstable();

        let mut ans = Vec::new();
        let mut prefix = String::new();

        for ch in search_word.chars() {
            prefix.push(ch);

            let idx = match products.binary_search_by(|p| p.as_str().cmp(&prefix)) {
                Ok(i) => i,
                Err(i) => i,
            };

            let mut suggestions = Vec::new();
            for i in idx..min(idx + 3, products.len()) {
                if products[i].starts_with(&prefix) {
                    suggestions.push(products[i].clone());
                } else {
                    break;
                }
            }
            ans.push(suggestions);
        }
        ans
    }
}

fn main() {
    let products = vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
    let search_word = String::from("mouse");

    println!("{:?}", Solution::suggested_products(products, search_word));
}

/*
1. Sorting Products
Time: O(n log n) (lexicographical sort of n strings)

2. Processing Each Prefix (total k prefixes)
For each prefix:
Binary search: O(log n)

At most 3 prefix matches checked: O(3 * m) = O(m) (to check starts_with)
So, per prefix: O(log n + m)

Total for all k prefixes: O(k * (log n + m))
*/