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

impl ListNode {
    fn wrap(self) -> Option<Box<Self>> {
        Some(Box::new(self))
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut res = &mut result;

        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut cache = 0;

        while l1.is_some() || l2.is_some() || cache != 0 {
            match (l1, l2) {
                (Some(ll1), Some(ll2)) => {
                    let value = ll1.val + ll2.val + cache;
                    cache = value / 10;
                    res.next = ListNode::new(value % 10).wrap();
                    res = res.next.as_mut().unwrap();
                    l1 = &ll1.next;
                    l2 = &ll2.next;
                }
                (Some(l), None) | (None, Some(l)) => {
                    let value = l.val + cache;
                    cache = value / 10;
                    res.next = ListNode::new(value % 10).wrap();
                    res = res.next.as_mut().unwrap();
                    l1 = &l.next;
                    l2 = &None;
                }
                (None, None) => {
                    res.next = ListNode::new(cache).wrap();
                    cache = 0;
                }
            }
        }

        return result.next;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_solution_1() {
        assert_eq!(Solution::add_two_numbers(None, None), None);
    }
}
