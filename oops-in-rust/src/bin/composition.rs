struct Address {
    city: String,
    zip: u32,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let address = Address {
        city: "San Francisco".to_string(),
        zip: 94107,
    };
    let person = Person {
        name: "Alice".to_string(),
        address,
    };
    println!("{} lives in {}", person.name, person.address.city);
}

/*
Rust prefers composition rather than inheritance by embedding structs within other structs.

Composition allows building complex types by combining simple ones.

More flexible and modular than classical inheritance.
*/