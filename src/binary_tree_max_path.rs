use std::cell::RefCell;

/**
 * Given the root of a binary tree, return the maximum path sum of any non-empty path.
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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
// #[cfg(test)]
pub struct Solution {}
/// Each node calculates longest paths through its children nodes and checks if sum of these paths is greater then currently found greatest sum.
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(ref node) => {
                let mut longest = RefCell::borrow(node).val;
                Solution::max_path_with_root(Rc::clone(node), &mut longest);
                longest
            }
        }
    }

    /// Returns longest path which roots in `node`. Updates `longest` if node is root of longest path
    fn max_path_with_root(node: Rc<RefCell<TreeNode>>, longest: &mut i32) -> i32 {
        let mut longest_left: i32 = match RefCell::borrow(&node).left {
            None => 0,
            Some(ref left) => Solution::max_path_with_root(Rc::clone(left), longest),
        };
        if longest_left < 0 {
            longest_left = 0;
        }
        let mut longest_right: i32 = match RefCell::borrow(&node).right {
            None => 0,
            Some(ref right) => Solution::max_path_with_root(Rc::clone(right), longest),
        };
        if longest_right < 0 {
            longest_right = 0
        }
        if longest_left + longest_right + RefCell::borrow(&node).val > *longest {
            *longest = longest_left + longest_right + RefCell::borrow(&node).val;
        }
        match longest_left > longest_right {
            true => longest_left + RefCell::borrow(&node).val,
            false => longest_right + RefCell::borrow(&node).val,
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
        0
    );
    let left = TreeNode {
        val: 2,
        left: None,
        right: None,
    };
    let right = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    };
    assert_eq!(
        Solution::max_path_sum(Some(Rc::from(RefCell::from(root)))),
        6
    );
    assert_eq!(
        Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode::new(-3))))),
        -3
    )
}
