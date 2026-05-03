use std::rc::Rc;
use std::cell::RefCell;

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
    fn detect_cycle_index(head: Option<Rc<RefCell<ListNode>>>) -> i32 {
        if head.is_none() {
            return -1;
        }

        let mut slow = head.clone();
        let mut fast = head.clone();
        let mut has_cycle = false;

        // Phase 1: detect cycle
        while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
            slow = s.borrow().next.clone();

            fast = f.borrow().next.clone();
            if let Some(next) = fast.clone() {
                fast = next.borrow().next.clone();
            } else {
                break;
            }

            if let (Some(s_ptr), Some(f_ptr)) = (&slow, &fast) {
                if Rc::ptr_eq(s_ptr, f_ptr) {
                    has_cycle = true;
                    break;
                }
            }
        }

        if !has_cycle {
            return -1;
        }

        // Phase 2: find cycle start node
        slow = head.clone();

        while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
            if Rc::ptr_eq(&s, &f) {
                // Phase 3: compute index
                let mut idx = 0;
                let mut cur = head.clone();

                while let Some(node) = cur {
                    if Rc::ptr_eq(&node, &s) {
                        return idx;
                    }
                    cur = node.borrow().next.clone();
                    idx += 1;
                }
            }

            slow = s.borrow().next.clone();
            fast = f.borrow().next.clone();
        }

        -1
    }
}

fn main() {
    let n1 = ListNode::new(3);
    let n2 = ListNode::new(2);
    let n3 = ListNode::new(0);
    let n4 = ListNode::new(-4);

    // 3 → 2 → 0 → -4
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());

    // cycle: -4 → 2 (index 1)
    n4.borrow_mut().next = Some(n2.clone());

    let head = Some(n1);

    println!("tail connects to node index {}", Solution::detect_cycle_index(head));
}