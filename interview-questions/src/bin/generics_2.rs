/*
Trait bounds restrict the types that can be used with generics,
ensuring they implement specific traits.
*/
fn largest<T: PartialOrd>(list: &[T]) -> &T {
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
    println!("Largest number: {}", largest(&numbers));

    let words = vec!["day", "dice", "dairy", "date"];
    println!("Largest word: {}", largest(&words));
}
li