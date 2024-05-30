# 145. Binary Tree Postorder Traversal
- [Submission](https://leetcode.com/submissions/detail/1113864510/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
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
pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut traversed = vec![];
    let mut stack = vec![];
    loop {
        if root.is_some() {
            let node = root.unwrap();
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            stack.push(node);
            if right.is_some() {
                stack.push(right.unwrap());
            }
            root = left;
        } else if !stack.is_empty() {
            let node = stack.pop().unwrap();
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if left.is_none() && right.is_none() {
                traversed.push(node.borrow().val);
            } else if right.is_some() {
                stack.push(node);
                stack.push(right.unwrap());
            } else {
                stack.push(node);
            }
            root = left;
        } else {
            break;
        }
    }
    traversed
}
}
```
