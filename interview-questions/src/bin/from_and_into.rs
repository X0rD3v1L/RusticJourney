#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

impl From<(String, u32)> for Person {
    fn from(data: (String, u32)) -> Self {
        Person {
            name : data.0,
            age: data.1
        }
    }
}

// impl Into<Person> for (String, u32) {
//     fn into(self) -> Person {
//         Person {
//             name: self.0,
//             age: self.1,
//         }
//     }
// }


fn main() {
    // Using `From`
    let person_data: (String, u32) = (String::from("Ben"), 24);
    let person = Person::from(person_data);
    println!("Name: {}, Age: {}", person.name, person.age);

    // Using `Into`
    let another_person_data = (String::from("Bob"), 23);
    let another_person: Person = another_person_data.into();
    println!("{:?}", another_person);
}