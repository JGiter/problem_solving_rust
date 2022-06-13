use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn add(&mut self, val: i32) {
        if val < self.val {
            match self.left {
                None => self.left = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(ref node) => node.borrow_mut().add(val),
            }
        }

        if val > self.val {
            match self.right {
                None => self.right = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(ref node) => node.borrow_mut().add(val),
            }
        }
    }
}

impl TreeNode {
    fn sorted(&self) -> Vec<i32> {
        let mut s = Vec::new();
        if let Some(ref left) = self.left {
            s.append(&mut left.borrow_mut().sorted())
        }
        s.push(self.val);
        if let Some(ref right) = self.right {
            s.append(&mut right.borrow_mut().sorted())
        }
        s
    }
}

fn bst_merge_sort(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut s: Vec<i32> = Vec::new();
    if let Some(left) = root1 {
        s.append(&mut left.borrow_mut().sorted())
    }
    if let Some(right) = root2 {
        s.append(&mut right.borrow_mut().sorted())
    }
    s.sort();
    return s;
}

#[test]
fn test() {
    let mut root1 = TreeNode::new(2);
    root1.add(1);
    root1.add(4);

    let mut root2 = TreeNode::new(1);
    root2.add(0);
    root2.add(3);

    assert_eq!(
        bst_merge_sort(
            Some(Rc::new(RefCell::new(root1))),
            Some(Rc::new(RefCell::new(root2)))
        ),
        vec![0, 1, 1, 2, 3, 4]
    )
}
