fn second_largest_element(arr: &[i32]) -> Option<i32> {
    if arr.len() < 2 {
        return None; // Not enough elements for a second largest
    }

    let mut first = i32::MIN;
    let mut second = i32::MIN;
 
    for &num in arr {
        if num > first {
            second = first;
            first = num;
        } else if num > second && num < first {
            second = num;
        }
    }

    if second == i32::MIN {
        None // No second largest found
    } else {
        Some(second)
    }

    //Time complexity is O(n) since we traverse the array once.
    //Space complexity is O(1) since we use only a fixed amount of extra space
}

// fn second_largest_element(arr: &[i32]) -> Option<i32> {
//     if arr.len() < 2 {
//         return None; // Not enough elements
//     }

//     let mut sorted = arr.to_vec();
//     sorted.sort_unstable();

//     let largest = sorted[sorted.len() - 1];
//     for &num in sorted.iter().rev().skip(1) {
//         if num != largest {
//             return Some(num);
//         }
//     }

//     None // All elements are equal
//     //Time is O(n log n) due to sorting, space is O(n) for the sorted vector
// }



fn main(){
    let arr = vec![1, 2, 3, 4, 5];
    let second_largest = second_largest_element(&arr);
    
    match second_largest {
        Some(value) => println!("The second largest element is: {}", value),
        None => println!("The array does not have a second largest element."),
    }
}