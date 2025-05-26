use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Rc<RefCell<Task>>>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        let rc_task = Rc::new(RefCell::new(task));
        self.todos.push(rc_task);
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task_rc) = self.todos.get(index) {
            task_rc.borrow_mut().completed = true;
        } else {
            println!("Invalid index {}", index);
        }
    }

    fn print_tasks(&self) {
        for (i, task_rc) in self.todos.iter().enumerate() {
            let task = task_rc.borrow();
            let status = if task.completed { "✓" } else { "✗" };
            println!("{}: [{}] {}", i, status, task.description);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let task = Task {
        description: "Learn Rust".to_owned(),
        completed: false,
    };

    todo_list.add_task(task);
    todo_list.complete_task(0);
    todo_list.print_tasks();
}
