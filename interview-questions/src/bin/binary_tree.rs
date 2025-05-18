// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
        if root.is_none() {
            *root = Some(new_node);
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());

        while let Some(current) = queue.pop_front() {
            let mut current_borrow = current.borrow_mut();

            if current_borrow.left.is_none() {
                current_borrow.left = Some(new_node);
                return;
            } else {
                queue.push_back(current_borrow.left.as_ref().unwrap().clone());
            }

            if current_borrow.right.is_none() {
                current_borrow.right = Some(new_node);
                return;
            } else {
                queue.push_back(current_borrow.right.as_ref().unwrap().clone());
            }
        }
    }

    pub fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(node) => {
                let node_borrow = node.borrow();
                Self::print_tree(&node_borrow.left);
                println!("{}", node_borrow.val);
                Self::print_tree(&node_borrow.right);
            }
            None => {}
        }
    }
}

fn main() {
    let mut root = None;
    for val in 1..=7 {
        TreeNode::insert(&mut root, val);
    }
    println!("Binary Tree:");
    TreeNode::print_tree(&root);
}
