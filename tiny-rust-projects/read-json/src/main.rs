use serde::{Serialize, Deserialize};

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
    let json = r#"
    {
        "brand": "BMW",
        "bikes": [
            {
                "model": "R1250GS",
                "year": 2023,
                "engine_size": 1254.0
            },
            {
                "model": "S1000RR",
                "year": 2022,
                "engine_size": 999.0
            },
            {
                "model": "F900XR",
                "year": 2021,
                "engine_size": 895.0
            },
            {
                "model": "G310GS",
                "year": 2023,
                "engine_size": 313.0
            }
        ]
    }"#;

    let parsed: BMWBike = serde_json::from_str(json).unwrap();
    println!("Brand of my BMW bike: {}", parsed.bikes[3].model);
}

