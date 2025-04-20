use std::collections::VecDeque;

struct StockPriceWindow {
    window: VecDeque<i32>,
    capacity: usize,
    sum: i64,
}

impl StockPriceWindow {
    fn new(k: usize) -> Self {
        Self {
            window: VecDeque::with_capacity(k),
            capacity: k,
            sum: 0,
        }
    }

    fn insert(&mut self, price: i32) {
        self.window.push_back(price);
        self.sum += price as i64;

        // Remove oldest if window exceeds capacity
        if self.window.len() > self.capacity {
            if let Some(removed) = self.window.pop_front() {
                self.sum -= removed as i64;
            }
        }
    }

    fn get_min(&self) -> Option<i32> {
        self.window.iter().copied().min()
    }

    fn get_max(&self) -> Option<i32> {
        self.window.iter().copied().max()
    }

    fn get_avg(&self) -> Option<f64> {
        if self.window.is_empty() {
            None
        } else {
            let avg = self.sum as f64 / self.window.len() as f64;
            Some((avg * 100.0).round() / 100.0)
        }
    }
}

fn main() {
    let mut spw = StockPriceWindow::new(3);

    spw.insert(10);
    spw.insert(20);
    spw.insert(30);
    

    println!("Min: {:?}", spw.get_min()); // 10
    println!("Max: {:?}", spw.get_max()); // 30
    println!("Avg: {:?}", spw.get_avg()); // 20.0

    spw.insert(40); // 10 gets evicted

    println!("Min: {:?}", spw.get_min()); // 20
    println!("Max: {:?}", spw.get_max()); // 40
    println!("Avg: {:?}", spw.get_avg()); // 30.0
}
