fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
struct Solution {}

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
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut res = ListNode::new(114514);
        let mut res_tail = &mut res;
        while list1.is_some() || list2.is_some() {
            if list1.is_none() {
                res_tail.next = list2.take();
                res_tail = res_tail.next.as_mut().unwrap();
                list2 = res_tail.next.take();
                continue;
            }
            if list2.is_none() {
                res_tail.next = list1.take();
                res_tail = res_tail.next.as_mut().unwrap();
                list1 = res_tail.next.take();
                continue;
            }
            if list1.as_ref().unwrap().val >= list2.as_ref().unwrap().val {
                res_tail.next = list2.take();
                res_tail = res_tail.next.as_mut().unwrap();
                list2 = res_tail.next.take();
            } else {
                res_tail.next = list1.take();
                res_tail = res_tail.next.as_mut().unwrap();
                list1 = res_tail.next.take();
            }
        }
        return res.next;
    }
}
