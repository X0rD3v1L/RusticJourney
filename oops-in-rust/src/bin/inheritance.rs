trait Animal {
    fn speak(&self);
}

trait Mammal: Animal {
    fn walk(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Mammal for Dog {
    fn walk(&self) {
        println!("Dog is walking.");
    }
}

fn main() {
    let dog = Dog;
    dog.speak();
    dog.walk();
}

/*
Rust doesn’t support classical inheritance. Instead, it uses trait inheritance.
*/