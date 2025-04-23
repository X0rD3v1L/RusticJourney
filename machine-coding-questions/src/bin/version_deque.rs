use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct VersionQueue {
    versions: Vec<VecDeque<i32>>,
}

impl VersionQueue {
    fn new() -> Self {
        VersionQueue {
            versions: vec![VecDeque::new()], // version 0: empty queue
        }
    }

    fn enqueue(&mut self, value: i32) {
        let mut new_queue = self.versions.last().unwrap().clone();
        new_queue.push_back(value);
        self.versions.push(new_queue);
    }

    fn dequeue(&mut self) -> Option<i32> {
        let mut new_queue = self.versions.last().unwrap().clone();
        let result = new_queue.pop_front();
        self.versions.push(new_queue);
        result
    }

    fn print(&self, version: usize) {
        if version >= self.versions.len() {
            println!("Version {} does not exist.", version);
        } else {
            let queue = &self.versions[version];
            let elements: Vec<String> = queue.iter().map(|e| e.to_string()).collect();
            println!("Version {}: [{}]", version, elements.join(", "));
        }
    }

    fn current_version(&self) -> usize {
        self.versions.len() - 1
    }
}

fn main() {
    let mut vq = VersionQueue::new();

    vq.enqueue(1); // version 1
    vq.enqueue(2); // version 2
    vq.enqueue(3); // version 3
    vq.dequeue();  // version 4
    vq.enqueue(4); // version 5

    println!("Current version: {}", vq.current_version());

    for v in 1..=vq.current_version() {
        vq.print(v);
    }
}
