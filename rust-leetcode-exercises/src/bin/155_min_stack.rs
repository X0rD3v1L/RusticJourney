struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(
            match self.stack.last() {
                Some(&(_, min)) => {
                    (val, min.min(val))
                },
                None => (val, val),  
            }
        )
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        if let Some(&(value, _)) = self.stack.last() {
            value
        } else {
            -1
        }
    }
    
    
    fn get_min(&self) -> i32 {
        if let Some(&(_, min)) = self.stack.last() {
            min
        } else {
            -1
        }
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(2);
    obj.pop();
    let ret_3 = obj.top();
    let ret_4 = obj.get_min();

    println!("Top :: {}\nMin :: {}", ret_3, ret_4);
}
