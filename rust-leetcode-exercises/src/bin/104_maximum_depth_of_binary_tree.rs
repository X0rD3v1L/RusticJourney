//Definition of a binary tree node

use std::rc::Rc;
use std::cell::RefCell;

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
}

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = Solution::max_depth(node.borrow().left.clone());
                let right_depth = Solution::max_depth(node.borrow().right.clone());
                std::cmp::max(left_depth, right_depth) + 1
            }
            None => 0,
        }
    }
}

fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nodes: &[Option<i32>], i: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if i < nodes.len() {
            if let Some(val) = nodes[i] {
                let left = helper(nodes, 2 * i + 1);
                let right = helper(nodes, 2 * i + 2);
                return Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left,
                    right,
                })));
            }
        }
        None
    }
    helper(nodes, 0)
}

fn main() {
    // let left = Rc::new(RefCell::new(TreeNode::new(2)));
    // let right = Rc::new(RefCell::new(TreeNode::new(3)));
    // let root = Rc::new(RefCell::new(TreeNode {
    //     val : 1,
    //     left: Some(left),
    //     right: Some(right),
    // }));

    // let depth = Solution::max_depth(Some(root));
    let nodes = vec![
        Some(3), Some(9), Some(20), None, None, Some(15), Some(7)
    ];
    let root = build_tree(&nodes);
    let depth = Solution::max_depth(root);
    println!("Max Depth :: {}", depth);
}