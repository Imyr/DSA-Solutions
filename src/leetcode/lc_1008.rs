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

pub fn bt_from_pre_in(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.len() == 0 {
        return None
    }

    let idx = inorder.binary_search(&preorder[0]).unwrap();
    let (left_inorder, right_inorder) = (inorder[0..idx].to_vec(), inorder[idx+1..inorder.len()].to_vec());
    let (left_preorder, right_preorder) = (preorder[1..=left_inorder.len()].to_vec(), preorder[left_inorder.len()+1..preorder.len()].to_vec());

    let mut node = TreeNode::new(preorder[0]);
    node.left = bt_from_pre_in(left_preorder, left_inorder);
    node.right = bt_from_pre_in(right_preorder, right_inorder);
    
    Some(Rc::new(RefCell::new(node)))
}

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut inorder = preorder.clone();
    inorder.sort();
    bt_from_pre_in(preorder, inorder)
}