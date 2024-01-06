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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        if val < node.borrow().val {
            if node.borrow().left.is_none() {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            } else {
                let nude = node.borrow_mut().left.take();
                node.borrow_mut().left = insert_into_bst(nude, val);
            }
        } else if val > node.borrow().val {
            if node.borrow().right.is_none() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            } else {
                let nude = node.borrow_mut().right.take();
                node.borrow_mut().right = insert_into_bst(nude, val);
            }
        } else {
            unreachable!()
        }
        return Some(node)
    } else {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}
