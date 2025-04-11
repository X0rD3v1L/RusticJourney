use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    label: String,
    children: Vec<Box<Node>>,
    parent: Option<*mut Node>, // raw pointer for cyclic reference
    ancestor_locked: usize,
    descendant_locked: usize,
    user_id: usize,
    is_locked: bool,
}

impl Node {
    fn new(label: &str, parent: Option<*mut Node>) -> Self {
        Self {
            label: label.to_string(),
            children: Vec::new(),
            parent,
            ancestor_locked: 0,
            descendant_locked: 0,
            user_id: 0,
            is_locked: false,
        }
    }
}

struct LockingTree {
    root: Box<Node>,
    label_to_node: HashMap<String, *mut Node>,
    output_log: Vec<String>,
}

impl LockingTree {
    fn new(root: Box<Node>) -> Self {
        let mut tree = Self {
            label_to_node: HashMap::new(),
            output_log: Vec::new(),
            root,
        };
        let root_ptr = tree.root.as_mut() as *mut Node;
        tree.fill_label_to_node(unsafe { &mut *root_ptr });
        tree
    }

    fn fill_label_to_node(&mut self, node: &mut Node) {
        self.label_to_node
            .insert(node.label.clone(), node as *mut _);
        for child in node.children.iter_mut() {
            self.fill_label_to_node(child.as_mut());
        }
    }

    fn update_descendants(&self, node: &mut Node, value: isize) {
        for child in node.children.iter_mut() {
            child.ancestor_locked = (child.ancestor_locked as isize + value) as usize;
            self.update_descendants(child, value);
        }
    }

    fn lock_node(&mut self, label: &str, id: usize) -> bool {
        let &node_ptr = self.label_to_node.get(label).unwrap();
        let node = unsafe { &mut *node_ptr };

        if node.is_locked || node.ancestor_locked > 0 || node.descendant_locked > 0 {
            return false;
        }

        let mut current = node.parent;
        while let Some(ptr) = current {
            unsafe {
                (*ptr).descendant_locked += 1;
                current = (*ptr).parent;
            }
        }

        self.update_descendants(node, 1);
        node.is_locked = true;
        node.user_id = id;
        true
    }

    fn unlock_node(&mut self, label: &str, id: usize) -> bool {
        let &node_ptr = self.label_to_node.get(label).unwrap();
        let node = unsafe { &mut *node_ptr };

        if !node.is_locked || node.user_id != id {
            return false;
        }

        let mut current = node.parent;
        while let Some(ptr) = current {
            unsafe {
                (*ptr).descendant_locked -= 1;
                current = (*ptr).parent;
            }
        }

        self.update_descendants(node, -1);
        node.is_locked = false;
        true
    }

    fn check_descendants_locked(
        &self,
        node: &mut Node,
        id: usize,
        locked_nodes: &mut Vec<*mut Node>,
    ) -> bool {
        if node.is_locked {
            if node.user_id != id {
                return false;
            }
            locked_nodes.push(node as *mut _);
        }

        for child in node.children.iter_mut() {
            if !self.check_descendants_locked(child, id, locked_nodes) {
                return false;
            }
        }

        true
    }

    fn upgrade_node(&mut self, label: &str, id: usize) -> bool {
        let &node_ptr = self.label_to_node.get(label).unwrap();
        let node = unsafe { &mut *node_ptr };

        if node.is_locked || node.ancestor_locked > 0 || node.descendant_locked == 0 {
            return false;
        }

        let mut locked_descendants = vec![];
        if !self.check_descendants_locked(node, id, &mut locked_descendants) {
            return false;
        }

        for &locked_node_ptr in &locked_descendants {
            let locked_node = unsafe { &mut *locked_node_ptr };
            self.unlock_node(&locked_node.label, id);
        }

        self.lock_node(label, id)
    }

    fn process_queries(&mut self, queries: Vec<(usize, String, usize)>) {
        for (op, label, id) in queries {
            let result = match op {
                1 => self.lock_node(&label, id),
                2 => self.unlock_node(&label, id),
                3 => self.upgrade_node(&label, id),
                _ => false,
            };
            self.output_log.push(result.to_string());
        }
    }

    fn get_output_log(&self) -> Vec<String> {
        self.output_log.clone()
    }
}

fn build_tree(root: &mut Box<Node>, child_count: usize, labels: &[String]) {
    let mut queue = VecDeque::new();
    queue.push_back(root.as_mut() as *mut Node);

    let mut index = 1;
    while let Some(current_ptr) = queue.pop_front() {
        if index >= labels.len() {
            break;
        }

        let current = unsafe { &mut *current_ptr };

        for _ in 0..child_count {
            if index >= labels.len() {
                break;
            }

            let mut child = Box::new(Node::new(&labels[index], Some(current_ptr)));
            queue.push_back(child.as_mut() as *mut Node);
            current.children.push(child);
            index += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let q: usize = lines.next().unwrap().parse().unwrap();

    let mut labels = Vec::with_capacity(n);
    for _ in 0..n {
        labels.push(lines.next().unwrap());
    }

    let mut root = Box::new(Node::new(&labels[0], None));
    build_tree(&mut root, m, &labels);

    let mut tree = LockingTree::new(root);

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let parts: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let opcode: usize = parts[0].parse().unwrap();
        let node = parts[1].clone();
        let user: usize = parts[2].parse().unwrap();
        queries.push((opcode, node, user));
    }

    tree.process_queries(queries);

    for output in tree.get_output_log() {
        println!("{}", output);
    }
}

