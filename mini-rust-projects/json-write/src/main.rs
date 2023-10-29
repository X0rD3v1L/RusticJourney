use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Bike {
    model: String,
    year: u32,
    engine_size: f64,
}

#[derive(Serialize, Deserialize)]
struct BMWBike {
    brand: String,
    bikes: Vec<Bike>,
}

fn main() {
    let bmw_bike: BMWBike = BMWBike {
        brand: String::from("BMW"),
        bikes: vec![
            Bike {
                model: String::from("R1250GS"),
                year: 2023,
                engine_size: 1254.0,
            },
            Bike {
                model: String::from("S1000RR"),
                year: 2022,
                engine_size: 999.0,
            },
            Bike {
                model: String::from("F900XR"),
                year: 2021,
                engine_size: 895.0,
            },
            Bike {
                model: String::from("G310GS"),
                year: 2023,
                engine_size: 313.0,
            }
        ],
    };

    let json = serde_json::to_string(&bmw_bike).unwrap();
    println!("JSON for BMW Bikes:\n{}", json);
}
