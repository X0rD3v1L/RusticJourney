use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Logger {
    logs: Rc<RefCell<Vec<String>>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            logs: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }

    fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}

fn main() {
    let logger = Logger::new();
    
    let logger1 = logger.clone();
    let logger2 = logger.clone();

    logger1.log("User signed in");
    logger2.log("User uploaded a file");

    // Shared state: both loggers updated the same underlying Vec
    for log in logger.get_logs() {
        println!("[LOG] {}", log);
    }
}
