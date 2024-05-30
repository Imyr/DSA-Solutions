# 2331. Evaluate Boolean Binary Tree
- [Submission](https://leetcode.com/submissions/detail/1259467534/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.2 MB |
```
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      let root = root.unwrap();
      let val = root.borrow().val;
      match val {
          0 => false,
          1 => true,
          2 => Solution::evaluate_tree(root.borrow_mut().left.take()) || Solution::evaluate_tree(root.borrow_mut().right.take()),
          3 => Solution::evaluate_tree(root.borrow_mut().left.take()) && Solution::evaluate_tree(root.borrow_mut().right.take()),
          _ => unreachable!()
      }
    }
}
```
