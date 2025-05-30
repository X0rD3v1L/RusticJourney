// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String
}

fn print(data: &str){
    println!("{:?}", data);
}
fn main() {
    let people = vec![
        Person {age : 24, name: "Ben".to_owned(), fav_color: "black".to_owned()},
        Person {age : 22, name: "Nava".to_owned(), fav_color: "blue".to_owned()},
        Person {age : 22, name: "Lal".to_owned(), fav_color: "black".to_owned()}
    ];

    for person in people {
        if person.age == 24 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}