use std::collections::{HashMap, HashSet};

struct  Solution;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut hash_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        
        hash_map.insert(stones[0] + 1, HashSet::from([1]));

        let stone_set: HashSet<i32> = stones.iter().cloned().collect();

        for &stone in stones.iter().skip(1) {
            if let Some(jumps) = hash_map.get(&stone).cloned() {
                for k in jumps {
                    for next_jump in [k - 1, k, k + 1] {
                        if next_jump > 0 {
                            let next_stone = stone + next_jump;
                            if stone_set.contains(&next_stone) {
                                hash_map.entry(next_stone)
                                    .or_insert(HashSet::new())
                                    .insert(next_jump);
                            }
                        }
                    }
                }
            }
        }
        hash_map.contains_key(stones.last().unwrap())
    }
}

fn main() {
    let stones = vec![0,1,3,5,6,8,12,17];
    println!("{}", Solution::can_cross(stones));
}