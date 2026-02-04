use std::collections::{HashSet};

fn count_digits(mut n: i32) -> i32 {
    let mut cnt = 0;
    while n > 0 {
        cnt += 1;
        n /= 10;
    }
    cnt
}

fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let (small, large) = if arr1.len() <= arr2.len() {
        (arr1, arr2)
    } else {
        (arr2, arr1)
    };

    let mut prefix_set = HashSet::new();

    for num in small {
        let mut x = num;
        while x != 0 && !prefix_set.contains(&x) {
            prefix_set.insert(x);
            x = x / 10;
        }
    }

    let mut res = 0;

    for num in large {
        let mut x = num;
        while x != 0 && !prefix_set.contains(&x) {
            x = x / 10;
        }

        if x != 0 {
            res = res.max(count_digits(x));
        }
    }

    res
}

fn main() {
    let arr1 = vec![1, 10, 100];
    let arr2 = vec![1000];

    let ans = longest_common_prefix(arr1, arr2);

    println!("{}", ans);
}