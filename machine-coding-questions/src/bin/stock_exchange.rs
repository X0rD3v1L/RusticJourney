/*
Build a Stock Exchange. Stock exchange takes in Buy and Sell orders and tries to match them based on their prices.
What is an order?
- a order is a request to Buy or Sell a stock at a given price.
What is a trade?
- a trade happens when a Buy order matches a Sell order.

When will a Buy order trade?
- when there is a Sell order in the market with Sell price ≤ Buy price
When will a Sell order trade?
- when there is a Buy order in the market with Buy price ≥ Sell price

(both these conditions are basically the same)

Sequence of events
- exchange receives order,
- checks if a trade is possible for the incoming order
    - if yes, remove both orders from exchange, and notify
    - else, retain the incoming order in the exchange (to possibly trade later)

In case multiple orders match with the incoming order, order with the better price gets the trade 
(Buy order with higher price gets preference, Sell order with lower price gets preference)

When does the user get notified?
- when an order is placed (notify the user)
- when a trade happens (notify both parties)

Notes:
- trade price will be determined by the Buy/Sell order which is already present in the market rather than the incoming order.
- for the sake of simplicity, each order will be of 1 qty.
- there can be many stocks that can be traded.
*/

use std::collections::{BTreeMap, HashMap};

/// OrderType defines whether the order is to Buy or Sell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Order struct represents a Buy or Sell request
#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub stock: String,
    pub price: u32,
    pub order_type: OrderType,
}

/// Trade struct represents a successful trade between two orders
#[derive(Debug, Clone)]
pub struct Trade {
    pub buy_order_id: u32,
    pub sell_order_id: u32,
    pub trade_price: u32,
}

/// StockExchange maintains the order book and processes trades
pub struct StockExchange {
    // Maps stock symbol to Buy and Sell order books
    pub buy_orders: HashMap<String, BTreeMap<u32, Vec<u32>>>,  // Price -> List of Order IDs (Descending order)
    pub sell_orders: HashMap<String, BTreeMap<u32, Vec<u32>>>, // Price -> List of Order IDs (Ascending order)
    pub orders: HashMap<u32, Order>,                          // Order ID -> Order Details
    pub next_order_id: u32,                                   // Auto-increment Order ID
}

impl StockExchange {
    /// Create a new StockExchange
    pub fn new() -> Self {
        Self {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),
            orders: HashMap::new(),
            next_order_id: 1,
        }
    }

    /// Place an order and process it
    pub fn place_order(&mut self, stock: &str, price: u32, order_type: OrderType) -> Vec<Trade> {
        let order_id = self.next_order_id;
        self.next_order_id += 1;

        let order = Order {
            order_id,
            stock: stock.to_string(),
            price,
            order_type,
        };

        println!("Order Placed: {:?}", order);
        self.orders.insert(order_id, order.clone());

        // Process the order and return the list of trades
        let trades = self.match_order(order);

        // Notify the user for each trade
        for trade in &trades {
            println!(
                "Trade Executed: Buy Order ID {}, Sell Order ID {}, Trade Price {}",
                trade.buy_order_id, trade.sell_order_id, trade.trade_price
            );
        }

        trades
    }

    /// Match an incoming order with the existing orders
    fn match_order(&mut self, order: Order) -> Vec<Trade> {
        let stock = &order.stock;
        let mut trades = Vec::new();

        match order.order_type {
            OrderType::Buy => {
                // Match against Sell orders
                if let Some(sell_book) = self.sell_orders.get_mut(stock) {
                    let mut matched_prices = Vec::new();

                    for (&sell_price, sell_order_ids) in sell_book.iter_mut() {
                        if sell_price > order.price {
                            break;
                        }

                        // Match the first available sell order
                        if let Some(sell_order_id) = sell_order_ids.pop() {
                            trades.push(Trade {
                                buy_order_id: order.order_id,
                                sell_order_id,
                                trade_price: sell_price,
                            });

                            // Remove the sell order if no orders are left at this price
                            if sell_order_ids.is_empty() {
                                matched_prices.push(sell_price);
                            }
                            break;
                        }
                    }

                    // Remove matched prices from the sell book
                    for price in matched_prices {
                        sell_book.remove(&price);
                    }
                }
            }
            OrderType::Sell => {
                // Match against Buy orders
                if let Some(buy_book) = self.buy_orders.get_mut(stock) {
                    let mut matched_prices = Vec::new();

                    for (&buy_price, buy_order_ids) in buy_book.iter_mut().rev() {
                        if buy_price < order.price {
                            break;
                        }

                        // Match the first available buy order
                        if let Some(buy_order_id) = buy_order_ids.pop() {
                            trades.push(Trade {
                                buy_order_id,
                                sell_order_id: order.order_id,
                                trade_price: buy_price,
                            });

                            // Remove the buy order if no orders are left at this price
                            if buy_order_ids.is_empty() {
                                matched_prices.push(buy_price);
                            }
                            break;
                        }
                    }

                    // Remove matched prices from the buy book
                    for price in matched_prices {
                        buy_book.remove(&price);
                    }
                }
            }
        }

        // If no trade occurred, add the order to the order book
        if trades.is_empty() {
            self.add_order_to_book(order);
        }

        trades
    }

    /// Add an order to the order book
    fn add_order_to_book(&mut self, order: Order) {
        let stock = &order.stock;
        let order_book = match order.order_type {
            OrderType::Buy => self.buy_orders.entry(stock.clone()).or_default(),
            OrderType::Sell => self.sell_orders.entry(stock.clone()).or_default(),
        };

        order_book.entry(order.price).or_default().push(order.order_id);
    }
}

fn main() {
    let mut exchange = StockExchange::new();

    println!("--- Test Case 1: Buy Order Trades with Existing Sell Order ---");
    // Place a Sell order
    exchange.place_order("AAPL", 100, OrderType::Sell);
    // Place a Buy order that matches the Sell order
    exchange.place_order("AAPL", 100, OrderType::Buy);
    println!();

    println!("--- Test Case 2: Sell Order Trades with Existing Buy Order ---");
    // Place a Buy order
    exchange.place_order("GOOGL", 200, OrderType::Buy);
    // Place a Sell order that matches the Buy order
    exchange.place_order("GOOGL", 200, OrderType::Sell);
    println!();

    println!("--- Test Case 3: No Trade Occurs ---");
    // Place a Buy order
    exchange.place_order("MSFT", 150, OrderType::Buy);
    // Place a Sell order that does not match
    exchange.place_order("MSFT", 180, OrderType::Sell);
    println!();

    println!("--- Test Case 4: Buy Order Matches Multiple Sell Orders ---");
    // Place multiple Sell orders
    exchange.place_order("TSLA", 300, OrderType::Sell);
    exchange.place_order("TSLA", 290, OrderType::Sell);
    // Place a Buy order that matches the best Sell order
    exchange.place_order("TSLA", 300, OrderType::Buy);
    println!();

    println!("--- Test Case 5: Sell Order Matches Multiple Buy Orders ---");
    // Place multiple Buy orders
    exchange.place_order("AMZN", 400, OrderType::Buy);
    exchange.place_order("AMZN", 410, OrderType::Buy);
    // Place a Sell order that matches the best Buy order
    exchange.place_order("AMZN", 400, OrderType::Sell);
    println!();

    println!("--- Test Case 6: Complex Sequence of Orders ---");
    // Place Buy and Sell orders for the same stock with complex scenarios
    exchange.place_order("NFLX", 500, OrderType::Buy);
    exchange.place_order("NFLX", 510, OrderType::Buy);
    exchange.place_order("NFLX", 490, OrderType::Sell);
    exchange.place_order("NFLX", 500, OrderType::Sell);
    exchange.place_order("NFLX", 515, OrderType::Buy);
    println!();

    println!("--- Test Case 7: Orders for Multiple Stocks ---");
    // Place Buy and Sell orders for different stocks
    exchange.place_order("META", 600, OrderType::Buy);
    exchange.place_order("META", 590, OrderType::Sell);
    exchange.place_order("IBM", 700, OrderType::Sell);
    exchange.place_order("IBM", 720, OrderType::Buy);
    exchange.place_order("MSFT", 190, OrderType::Buy);
    println!();
}
