use crate::math::utils::print_result;

pub fn add(a: i32, b: i32) -> i32 {
    let result = a + b;
    print_result("Add result", result);
    result
}
