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
    pub fn middle_node(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
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
    let head = build_list(nums);

    println!("Original list:");
    print_list(&head);

    let result = Solution::middle_node(head);

    println!("Middle of the Linked List is :: {}", &result.unwrap().val);
}