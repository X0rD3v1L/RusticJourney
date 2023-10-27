use std::collections::HashMap;

fn main(){
    //Creating a New Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    let _field_name = String::from("Favorite color");
    let _field_value = String::from("Blue");

    let mut _map = HashMap::new();
    _map.insert(_field_name, _field_value);

}