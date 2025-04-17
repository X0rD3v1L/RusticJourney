// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    // Create nodes from tail to head
    let node3 = Some(Box::new(ListNode::new(3)));
    let mut node2 = ListNode::new(2);
    node2.next = node3;

    let mut node1 = ListNode::new(1);
    node1.next = Some(Box::new(node2));

    // Head of the linked list
    let head = Some(Box::new(node1));

    // Print the list
    print_list(&head);

    println!("{}", sum_nodes(&head));
}

// Helper function to print the linked list
fn print_list(mut node: &Option<Box<ListNode>>) {
    while let Some(n) = node {
        print!("{} -> ", n.val);
        node = &n.next;
    }
    println!("None");
}

//Function to sum the digits which are represented as nodes of the linked list
fn sum_nodes(mut node: &Option<Box<ListNode>>) -> i32{
    let mut total = 0;
    while let Some(n) = node {
        total += n.val;
        node = &n.next;
    }

    total
}
