use rand::{rng, Rng};
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MarketData {
    symbol: String,
    price: f64,
    is_corrupt: bool,
}

// Connect to fast line and receive one market data update
async fn get_from_fast_line() -> MarketData {
    sleep(Duration::from_millis(50)).await; // Simulate low latency
    MarketData {
        symbol: "BTC/USD".to_string(),
        price: rng().random_range(100.0..200.0),
        is_corrupt: rng().random_bool(0.1), // 10% chance of being corrupt
    }
}

// Connect to reliable line and receive one accurate update
async fn get_from_reliable_line() -> MarketData {
    sleep(Duration::from_millis(500)).await; // Simulate delay
    MarketData {
        symbol: "BTC/USD".to_string(),
        price: rng().random_range(100.0..200.0),
        is_corrupt: false,
    }
}

fn is_data_corrupt(data: &MarketData) -> bool {
    data.is_corrupt
}

async fn market_data_loop() {
    for _ in 0..25 {
        // Simulate connecting to fast line
        println!("🔌 Connecting to Fast Line...");
        let fast_data = get_from_fast_line().await;
        println!("[Fast Line] Received: {:?}", fast_data);

        if is_data_corrupt(&fast_data) {
            println!("❌ Corrupt data detected! Switching to Reliable Line...");

            // Simulate disconnect and switch
            println!("🔌 Disconnecting from Fast Line...");
            println!("🔌 Connecting to Reliable Line...");
            let correct_data = get_from_reliable_line().await;
            println!("✅ [Reliable Line] Corrected Data: {:?}", correct_data);

            println!("🔄 Switching back to Fast Line...\n");
        } else {
            println!("✅ [Fast Line] Data OK → Processing: {:?}", fast_data);
        }
    }
}

#[tokio::main]
async fn main() {
    market_data_loop().await;
}
