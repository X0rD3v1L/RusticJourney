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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // edge cases
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }

        // Step 1: find middle
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;

            fast = &fast.as_ref().unwrap().next;
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
            }
        }

        // Step 2: reverse second half
        let mut second_half = Solution::reverse_list(slow.clone());

        // Step 3: compare halves
        let mut first_half = head;

        while second_half.is_some() {
            if first_half.as_ref().unwrap().val != second_half.as_ref().unwrap().val {
                return false;
            }

            first_half = first_half.unwrap().next;
            second_half = second_half.unwrap().next;
        }

        true
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        prev
    }
}

// helper: build list
fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

// helper: print list
fn print_list(head: &Option<Box<ListNode>>) {
    let mut curr = head;
    while let Some(node) = curr {
        print!("{} -> ", node.val);
        curr = &node.next;
    }
    println!("None");
}

fn main() {
    let nums = vec![1, 2, 2, 1];
    let head = build_list(nums);

    println!("List:");
    print_list(&head);

    let result = Solution::is_palindrome(head);
    println!("Is palindrome: {}", result);
}