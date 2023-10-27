fn main() {
    //Creating a New Vector

    let _v: Vec<i32> = Vec::new();
    let _u = vec![1,2,3];

    //Updating a vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //Iterating over the Values in a Vector

    for element in &v {
        println!("{element}");
    }

    let mut mutuable_vector = vec![1,2,3,4,5];

    for i in &mut mutuable_vector {
        *i += 50;
    }

    for element in &mutuable_vector {
        println!("{element}");
    }


}