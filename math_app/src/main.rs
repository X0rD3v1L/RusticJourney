use math_app::math::{add::add, sub::sub, utils::print_result};

fn main() {
    let a = 10;
    let b = 4;

    let sum = add(a, b);
    let diff = sub(a, b);

    print_result("Sum", sum);
    print_result("Difference", diff);
}
