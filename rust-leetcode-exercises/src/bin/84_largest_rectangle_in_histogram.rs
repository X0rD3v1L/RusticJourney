fn largest_rectangle_in_histogram(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let n = heights.len();
    let mut stack: Vec<(usize, i32)> = Vec::new();

    stack.push((0, heights[0]));

    for index in 1..n {
        if heights[index] < stack.last().unwrap().1 {
            let mut temp_index = 0;
            while !stack.is_empty() && heights[index] < stack.last().unwrap().1 {
                let top = stack.pop().unwrap();
                max_area = max_area.max(top.1 as i32 * (index - top.0) as i32);
                temp_index = top.0;
            }
            stack.push((temp_index, heights[index]));
        } else {
            stack.push((index, heights[index]));
        }
    }

    while !stack.is_empty() {
            let top = stack.pop().unwrap();
            max_area = max_area.max(top.1 as i32 * (n - top.0) as i32);
        }

    max_area
}

fn main() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let largest_rectange_area = largest_rectangle_in_histogram(heights);
    println!("{}", largest_rectange_area);
}