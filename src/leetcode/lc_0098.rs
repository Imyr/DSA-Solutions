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

pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut traversed = vec![];
    let mut stack = vec![];
    loop {
        if root.is_some() {
            let node = root.take().unwrap();
            root = node.borrow_mut().left.take();
            stack.push(node);
        } else if !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node.borrow_mut().right.take() {
                Some(right) => stack.push(right),
                None => {},
            };
            if node.borrow().left.is_some() {
                root = node.borrow_mut().left.take();
                stack.push(node);
            } else {
                traversed.push(node.borrow().val);
            }
        } else {
            break;
        }
    }
    let mut sorted_traversed = traversed.clone();
    sorted_traversed.sort_unstable();
    sorted_traversed.dedup();
    traversed == sorted_traversed
}