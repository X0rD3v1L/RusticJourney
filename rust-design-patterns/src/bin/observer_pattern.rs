use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug)]
struct PriceUpdate {
    symbol: String,
    price: f64,
}

// Simulates a market data feed publishing prices

async fn market_feed(publisher: broadcast::Sender<PriceUpdate>) {
    let prices = vec![
        ("AAPL", 192.34),
        ("AAPL", 193.20),
        ("GOOG", 2750.55),
        ("AAPL", 191.85),
    ];

    for (symbol, price) in prices {
        let update = PriceUpdate {
            symbol: symbol.to_string(),
            price,
        };
        println!("📡 MarketFeed: Broadcasting {:?}", update);
        let _ = publisher.send(update);
        sleep(Duration::from_secs(1)).await;
    }
}

// Mean Reversion Strategy
async fn mean_reversion(mut receiver: broadcast::Receiver<PriceUpdate>) {
    while let Ok(update) = receiver.recv().await {
        println!("📈 [MeanReversion] {} at ${:.2}: Checking deviation from mean", update.symbol, update.price);
    }
}

// Momentum Strategy
async fn momentum(mut receiver: broadcast::Receiver<PriceUpdate>) {
    while let Ok(update) = receiver.recv().await {
        println!("⚡ [Momentum] {} at ${:.2}: Detecting momentum trend", update.symbol, update.price);
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);

    // Clone receiver for each strategy
    let mean_rx = tx.subscribe();
    let mom_rx = tx.subscribe();

    // Spawn each observer strategy
    tokio::spawn(mean_reversion(mean_rx));
    tokio::spawn(momentum(mom_rx));

    // Simulate market feed
    market_feed(tx).await;

    // Give some time for strategies to process
    sleep(Duration::from_secs(2)).await;
}
