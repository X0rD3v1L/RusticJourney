/*
iter()
Behavior: Creates an iterator that borrows each element of the vector.

Ownership: Does not take ownership of the elements, just borrows them.

When to Use: When you want to iterate without consuming the original collection.
*/

fn main() {
    let arr = vec![0, 1, 2, 3, 4, 5];

    // //value is moved in this case
    // for elem in arr {
    //     println!("{}", elem);
    // }

    // //And returns an error as we are accessing a value which is already moved in the above loop
    // println!("{:?}", arr);

    //take a reference but not ownership
    for &elem in arr.iter() {
        println!("{}", &elem);
    }

    println!("{:?}", arr);

    // //takes ownership
    // for elem in arr.into_iter() {
    //     println!("{}", elem);
    // }

    // //here returns an error as we are accessing a value which is already moved
    // println!("{:?}", arr);

    /*
    Behavior: Creates an iterator that takes ownership of the vector and each element.

    Ownership: Moves the vector, consuming it. You cannot use numbers after this.

    When to Use: When you need to take ownership or transform elements without keeping the original collection.
    */

    // A vector of tasks
    let tasks = vec![
        String::from("Clean the house"),
        String::from("Buy groceries"),
        String::from("Complete Rust project"),
    ];

    // Using .into_iter() to move ownership of each element
    let processed_tasks: Vec<String> = tasks.into_iter()
        .map(|task| format!("Task: {}", task))  // Transforming the task
        .collect();  // Collecting into a new vector

    println!("Processed Tasks:");
    for task in processed_tasks {
        println!("{}", task);
    }

    // Uncommenting the next line will cause an error because 'tasks' is moved
    // println!("{:?}", tasks);

    //if we do the above operation using iter(), this would force us to clone each string 
    //to collect into a new vector, which is less efficient.
    
}