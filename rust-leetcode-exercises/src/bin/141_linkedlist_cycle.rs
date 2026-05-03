use std::rc::Rc;
use std::cell::RefCell;

// Definition for singly-linked list
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

struct Solution;

impl Solution {
    fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
            // move slow by 1
            slow = s.borrow().next.clone();

            // move fast by 2
            fast = f.borrow().next.clone();
            if let Some(next) = fast.clone() {
                fast = next.borrow().next.clone();
            } else {
                return false;
            }

            // check if they meet (same node)
            if let (Some(s_ptr), Some(f_ptr)) = (&slow, &fast) {
                if Rc::ptr_eq(s_ptr, f_ptr) {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    // Create nodes
    let n1 = ListNode::new(1);
    let n2 = ListNode::new(2);
    let n3 = ListNode::new(3);
    let n4 = ListNode::new(4);

    // Build list: 1 -> 2 -> 3 -> 4
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());

    // Create a cycle: 4 -> 2
    n4.borrow_mut().next = Some(n2.clone());

    let head = Some(n1);

    let has_cycle = Solution::has_cycle(head);
    println!("Cycle detected: {}", has_cycle);
}