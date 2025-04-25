use std::collections::{HashMap, BTreeMap};

struct TrieNode {
    children: HashMap<char, TrieNode>,
    top_k: Vec<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            top_k: Vec::new(),
        }
    }

    fn insert(&mut self, word: &str, popularity: &HashMap<String, i32>, k: usize) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
            node.update_top_k(word, popularity, k);
        }
    }

    fn update_top_k(&mut self, word: &str, popularity: &HashMap<String, i32>, k: usize) {
        if !self.top_k.contains(&word.to_string()) {
            self.top_k.push(word.to_string());
        }

        self.top_k.sort_by(|a, b| {
            let pa = popularity.get(a).unwrap_or(&0);
            let pb = popularity.get(b).unwrap_or(&0);
            pb.cmp(pa).then_with(|| a.cmp(b))
        });

        if self.top_k.len() > k {
            self.top_k.pop();
        }
    }
}

struct Solution;

impl Solution {
    pub fn suggested_products(
        products: Vec<String>,
        popularity: HashMap<String, i32>,
        search_word: String,
        k: usize,
    ) -> Vec<Vec<String>> {
        let mut root = TrieNode::new();

        // Insert products into the Trie
        for product in &products {
            root.insert(product, &popularity, k);
        }

        // Traverse the Trie with the search word
        let mut result = Vec::new();
        let mut node = &root;
        for ch in search_word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                result.push(next_node.top_k.clone());
                node = next_node;
            } else {
                // From this point, all suggestions will be empty
                while result.len() < search_word.len() {
                    result.push(Vec::new());
                }
                break;
            }
        }

        result
    }
}

fn main() {
    use std::collections::HashMap;

    let products = vec![
        "apple", "appetizer", "application", "app", "apply", "banana", "appstore",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    let mut popularity = HashMap::new();
    popularity.insert("apple".to_string(), 80);
    popularity.insert("appetizer".to_string(), 70);
    popularity.insert("application".to_string(), 90);
    popularity.insert("app".to_string(), 90);
    popularity.insert("apply".to_string(), 85);
    popularity.insert("banana".to_string(), 60);
    popularity.insert("appstore".to_string(), 90);

    let search_word = "app".to_string();
    let k = 3;

    let suggestions =
        Solution::suggested_products(products, popularity, search_word, k);

    for suggestions_for_prefix in suggestions {
        println!("{:?}", suggestions_for_prefix);
    }
}
/*
""      : [app, application, appstore]  
└── a   : [app, application, appstore]  
    └── p   : [app, application, appstore]  
        └── p   : [app, application, appstore]  
            ├── l   : [application, apple, apply]  
            │   └── e: [apple]  
            │   └── i: [application]  
            │   └── y: [apply]  
            ├── s   : [appstore]  
            │   └── t: [appstore]  
            └── e   : [appetizer]  
*/