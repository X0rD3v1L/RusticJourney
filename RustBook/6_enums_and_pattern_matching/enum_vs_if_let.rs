fn main() {
    let some_option = Some(5);

    match some_option {
        Some(value) => {
            println!("Got a value: {}", value);
        }
        None => { //Need to ensure exhaustive checking is done.
            println!("Got nothing.");
        }
    }

    let some_option = Some(10);

    //Exhaustive checking is optional
    if let Some(value) = some_option {
        println!("Got a value: {}", value);
    } else {
        println!("Got nothing.");
    }
}
