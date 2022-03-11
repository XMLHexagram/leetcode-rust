fn main() {
    println!("Hello, world!");
}

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

struct Solution {}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut walk = 0;

        match head {
            Some(head1) => {
                let mut node = &head1;
                while node.next.is_some() {
                    len += 1;
                    node = node.next.as_ref().unwrap();
                }
                walk = len - (k % len);
                node.next = head;

                let mut node = head1;
                let mut prv = &Box::new(ListNode::new(-1000));
                for _ in 1..walk {
                    // prv = &node;
                    node = node.next.unwrap();
                }
                // prv.next = None;
                return None;
            }
            None => {
                return None;
            }
        }
    }
}
