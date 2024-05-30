# 700. Search in a Binary Search Tree
- [Submission](https://leetcode.com/submissions/detail/1115014596/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.9 MB |
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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if val < node.borrow().val {
                    Solution::search_bst(node.borrow_mut().left.take(), val)
                } else if val > node.borrow().val {
                    Solution::search_bst(node.borrow_mut().right.take(), val)
                } else {
                    Some(node)
                }
            },
            None => None,
        }
    }
}
```
