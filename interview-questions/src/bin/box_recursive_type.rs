// Define a linked list where each node contains a value and a pointer to the next node
enum List {
    Cons(i32, Box<List>), // Each node contains a value and a pointer to the next node
    Nil,
}

fn main() {
    // Create a list: 1 -> 2 -> 3 -> Nil
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    // A simple way to print the list
    print_list(&list);
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} -> ", value);
            print_list(next);
        },
        List::Nil => println!("Nil"),
    }
}
