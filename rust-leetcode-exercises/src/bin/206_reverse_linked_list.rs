// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

fn main(){
    let third = ListNode {
        val : 3,
        next: None,
    };
    let second = ListNode {
        val : 2,
        next: Some(Box::new(third)),
    };
    let first = ListNode {
        val : 1,
        next: Some(Box::new(second)),
    };

    let head = Some(Box::new(first));
    let reversed_list = Solution::reverse_linked_list(head);

    println!("{:?}", reversed_list);
}