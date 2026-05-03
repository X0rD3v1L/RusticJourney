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
    pub fn get_intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {

        let mut cur_a = head_a.clone();
        let mut cur_b = head_b.clone();

        while match (&cur_a, &cur_b) {
            (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
            (None, None) => false,
            _ => true,
        } {
            cur_a = match cur_a {
                Some(node) => node.borrow().next.clone(),
                None => head_b.clone(),
            };

            cur_b = match cur_b {
                Some(node) => node.borrow().next.clone(),
                None => head_a.clone(),
            };
        }

        cur_a
    }
}

fn main() {
    // shared part
    let c1 = ListNode::new(8);
    let c2 = ListNode::new(4);
    let c3 = ListNode::new(5);

    c1.borrow_mut().next = Some(c2.clone());
    c2.borrow_mut().next = Some(c3.clone());

    // list A: 4 -> 1 -> 8 -> 4 -> 5
    let a1 = ListNode::new(4);
    let a2 = ListNode::new(1);
    a1.borrow_mut().next = Some(a2.clone());
    a2.borrow_mut().next = Some(c1.clone());

    // list B: 5 -> 6 -> 1 -> 8 -> 4 -> 5
    let b1 = ListNode::new(5);
    let b2 = ListNode::new(6);
    let b3 = ListNode::new(1);
    b1.borrow_mut().next = Some(b2.clone());
    b2.borrow_mut().next = Some(b3.clone());
    b3.borrow_mut().next = Some(c1.clone());

    let result = Solution::get_intersection_node(Some(a1), Some(b1));

    match result {
        Some(node) => println!("Intersection at: {}", node.borrow().val),
        None => println!("No intersection"),
    }
}