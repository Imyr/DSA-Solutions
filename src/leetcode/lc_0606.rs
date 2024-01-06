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

pub fn recursive_preorder(s: &mut Vec<String>, root: Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(node) => {
            s.push(node.borrow().val.to_string());
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            match (left, right) {
                (None, None) => return,
                (Some(node), None) => {
                    s.push("(".to_string());
                    recursive_preorder(s, Some(node));
                    s.push(")".to_string());
                }
                (None, Some(node)) => {
                    s.push("()(".to_string());
                    recursive_preorder(s, Some(node));
                    s.push(")".to_string());
                },
                (Some(node_l), Some(node_r)) => {
                    s.push("(".to_string());
                    recursive_preorder(s, Some(node_l));
                    s.push(")(".to_string());
                    recursive_preorder(s, Some(node_r));
                    s.push(")".to_string());
                }
            }
        },
        None => return,
    }

}

pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut traversed = vec![];
    recursive_preorder(&mut traversed, root);
    traversed.join("")
}