fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn apply_function(f : fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

fn main() {
    let add_res = apply_function(add, 5, 3);
    let sub_res = apply_function(sub, 10, 5);
    
    println!("{} {}", add_res, sub_res);
}

/*
Returning a function pointer from other functions

fn choose_function(op: &str) -> fn(i32, i32) -> i32 {
    match op {
        "add" => add,
        "subtract" => sub,
        _ => panic!("Unknown operation"),
    }
}

fn main() {
    let operation = choose_function("add");
    println!("Result: {}", operation(10, 5)); // Output: Result: 15
}
*/