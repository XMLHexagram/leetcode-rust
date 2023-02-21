fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::i32::MAX;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
            match root {
                Some(root) => {
                    // root.borrow().left
                    let (left, left_ok) = dfs(root.borrow().left.clone());
                    let (right, right_ok) = dfs(root.borrow().right.clone());
                    if (left - right).abs() > 1 || !left_ok || !right_ok {
                        return (-1, false);
                    }

                    return (left.max(right) + 1, true);
                }
                None => (0, true),
            }
        }

        let (_, ok) = dfs(root);
        return ok;
    }
}

mod test {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    use crate::Solution;
    use crate::TreeNode;

    #[test]
    fn test_solution_1() {
        let mut test_case = TreeNode::new(3);
        test_case.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        test_case.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        let res = Solution::is_balanced(Some(Rc::new(RefCell::new(test_case))));
        let test_case = array_to_node_tree(vec![Some(1), Some(1), Some(3)]);

        Solution::is_balanced(Some(Rc::new(RefCell::new(test_case))));
        assert_eq!(res, true)
    }

    fn array_to_node_tree(input: Vec<Option<i32>>) -> Option<TreeNode> {
        let input = input.clone();
        let mut parents: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut root: Option<TreeNode> = None;

        for i in 0..input.len() {
            if i == 0 {
                root = Some(TreeNode::new(input[i].unwrap_or(-1)));
                parents.push_back(Rc::new(RefCell::new(root.unwrap())));
                continue;
            }

            if !parents.is_empty() {
                {
                    let mut parent = parents.front().unwrap().borrow_mut();
                    if parent.left == None {
                        parent.left =
                            Some(Rc::new(RefCell::new(TreeNode::new(input[i].unwrap_or(-1)))));
                    }
                }
                let test = parents.pop_front().unwrap();
                let mut a = test.borrow_mut();
                a.right = Some(Rc::new(RefCell::new(TreeNode::new(input[i].unwrap_or(-1)))));
            } else {
                s
            }


        }

        return root;
    }
}
