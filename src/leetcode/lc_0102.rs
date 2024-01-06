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
use std::collections::VecDeque;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut traversed = vec![];
    let mut chotu = vec![];
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    queue1.push_back(root);
    loop {
        if queue1.is_empty() && queue2.is_empty() {break;}

        if !queue1.is_empty() {
            while !queue1.is_empty() {
                match queue1.pop_front().unwrap() {
                    Some(node) => {
                        chotu.push(node.borrow().val);
                        queue2.push_back(node.borrow_mut().left.take());
                        queue2.push_back(node.borrow_mut().right.take());
                    },
                    None => continue,
                }
            }
            if !chotu.is_empty() {
                traversed.push(chotu);
                chotu = vec![];
            }

        } else if !queue2.is_empty() {
            while !queue2.is_empty() {
                match queue2.pop_front().unwrap() {
                    Some(node) => {
                        chotu.push(node.borrow().val);
                        queue1.push_back(node.borrow_mut().left.take());
                        queue1.push_back(node.borrow_mut().right.take());
                    },
                    None => continue,
                }
            }
            if !chotu.is_empty() {
                traversed.push(chotu);
                chotu = vec![];
            }
        }
    }
    traversed
}