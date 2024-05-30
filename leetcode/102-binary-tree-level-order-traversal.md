# 102. Binary Tree Level Order Traversal
- [Submission](https://leetcode.com/submissions/detail/1142419702/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.5 MB |
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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut traversed = vec![];
        let mut chotu = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        loop {
            if !queue.is_empty() {
                for _ in 0..queue.len() {
                    match queue.pop_front().unwrap() {
                        Some(node) => {
                            chotu.push(node.borrow().val);
                            queue.push_back(node.borrow_mut().left.take());
                            queue.push_back(node.borrow_mut().right.take());
                        },
                        None => continue,
                    }
                }
                if !chotu.is_empty() {
                    traversed.push(chotu);
                    chotu = vec![];
                }
            } else {break;}
        } 
        traversed
    }
}
```
