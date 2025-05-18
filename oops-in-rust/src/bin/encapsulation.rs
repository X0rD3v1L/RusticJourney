mod animal {
    pub struct Dog {
        name: String,
        age: u8,
    }

    impl Dog {
        // Public constructor
        pub fn new(name: &str, age: u8) -> Dog {
            Dog {
                name: name.to_string(),
                age,
            }
        }

        // Public method
        pub fn speak(&self) {
            println!("Woof! My name is {}.", self.name);
        }

        // Private method
        fn secret(&self) {
            println!("I know a secret!");
        }
    }
}

fn main() {
    let dog = animal::Dog::new("Buddy", 3);
    dog.speak();
    // dog.secret(); // Error: private method, not accessible here
}
