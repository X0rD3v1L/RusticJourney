#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        // count length
        let mut len = 0;
        let mut curr = &dummy.next;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        // find node before target
        let mut curr = &mut dummy;
        for _ in 0..(len - n) {
            curr = curr.next.as_mut().unwrap();
        }

        // remove
        let next = curr.next.as_mut().unwrap().next.take();
        curr.next = next;

        dummy.next
    }
}

// helper: build linked list from vec
fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

// helper: print linked list
fn print_list(head: &Option<Box<ListNode>>) {
    let mut curr = head;
    while let Some(node) = curr {
        print!("{} -> ", node.val);
        curr = &node.next;
    }
    println!("None");
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let n = 2;

    let head = build_list(nums);

    println!("Original list:");
    print_list(&head);

    let result = Solution::remove_nth_from_end(head, n);

    println!("After removing {}th node from end:", n);
    print_list(&result);
}