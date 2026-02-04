fn find_majority_element(nums: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut candidate = 0;

    for &num in nums {
        if count == 0 {
            candidate = num;
        }
        if num == candidate{
            count += 1;
        } else {
            count -= 1;
        }
    }

    for &num in nums {
        if num == candidate {
            count += 1;
        }
    }
    if count > nums.len() as i32 / 2 {
        return candidate;
    } else {
        panic!("No majority element found");
    }
}
fn main() {
    let nums = vec![7, 7, 5, 7, 5, 1, 5, 7, 5, 5, 7, 7, 5, 5, 5, 5, 5];
    let majority_element = find_majority_element(&nums);
    println!("The majority element is: {}", majority_element);
}