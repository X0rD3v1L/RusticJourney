enum Car {
    Sedan(u32),   // Associated with an integer representing the number of doors
    SUV(u32),     // Associated with an integer representing the seating capacity
    SportsCar,    // No associated data
}

fn main() {
    let my_car = Car::Sedan(4);
    let _suv_car = Car::SUV(4);
    let other_car = Car::SportsCar;

    match my_car {
        Car::Sedan(doors) => println!("I have a sedan with {} doors.", doors),
        Car::SUV(capacity) => println!("I have an SUV with seating for {}.", capacity),
        Car::SportsCar => println!("I have a sports car."),
    }

    match other_car {
        Car::Sedan(doors) => println!("Other car is a sedan with {} doors.", doors),
        Car::SUV(capacity) => println!("Other car is an SUV with seating for {}.", capacity),
        Car::SportsCar => println!("Other car is a sports car."),
    }
}
