//! Observer pattern — subjects notify a dynamic set of observers without
//! knowing who they are. Here implemented with Tokio's `broadcast` channel:
//! the market feed is the *subject*; each strategy is an *observer*.

use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

// ── Domain types ──────────────────────────────────────────────────────────────

/// A price tick emitted by the market feed.
#[derive(Clone, Debug)]
struct PriceUpdate {
    symbol: String,
    price: f64,
}

// ── Subject ───────────────────────────────────────────────────────────────────

/// Broadcasts a fixed sequence of price updates with 1-second intervals.
async fn market_feed(publisher: broadcast::Sender<PriceUpdate>) {
    let prices = [
        ("AAPL", 192.34),
        ("AAPL", 193.20),
        ("GOOG", 2750.55),
        ("AAPL", 191.85),
    ];

    for (symbol, price) in prices {
        let update = PriceUpdate { symbol: symbol.to_owned(), price };
        println!("📡 MarketFeed: Broadcasting {:?}", update);
        // A send error only means there are no active receivers — safe to ignore.
        let _ = publisher.send(update);
        sleep(Duration::from_secs(1)).await;
    }
}

// ── Observers ─────────────────────────────────────────────────────────────────

async fn mean_reversion(mut rx: broadcast::Receiver<PriceUpdate>) {
    while let Ok(update) = rx.recv().await {
        println!(
            "📈 [MeanReversion] {} at ${:.2}: Checking deviation from mean",
            update.symbol, update.price
        );
    }
}

async fn momentum(mut rx: broadcast::Receiver<PriceUpdate>) {
    while let Ok(update) = rx.recv().await {
        println!(
            "⚡ [Momentum] {} at ${:.2}: Detecting momentum trend",
            update.symbol, update.price
        );
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);

    tokio::spawn(mean_reversion(tx.subscribe()));
    tokio::spawn(momentum(tx.subscribe()));

    market_feed(tx).await;

    // Give spawned tasks time to drain the remaining messages.
    sleep(Duration::from_secs(2)).await;
}
