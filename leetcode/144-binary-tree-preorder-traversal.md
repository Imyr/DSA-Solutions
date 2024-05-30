# 144. Binary Tree Preorder Traversal
- [Submission](https://leetcode.com/submissions/detail/1113124275/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
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
pub fn preorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut traversed = vec![];
    let mut stack = vec![];
    while root.is_some() {
        let node = Rc::try_unwrap(root.take().unwrap()).unwrap().into_inner();
        traversed.push(node.val);
        if node.right.is_some() {
            stack.push(node.right)
        }
        if node.left.is_some() {
            root = node.left
        } else if !stack.is_empty() {
            root = stack.pop().unwrap()
        }
    }
    traversed
}
}
```
