fn largest<T: Ord>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Largest: {}", largest(&numbers));

    let words = vec!["day", "dice", "dairy", "date"];
    println!("Largest word: {}", largest(&words));
}