# 98. Validate Binary Search Tree
- [Submission](https://leetcode.com/submissions/detail/1115783249/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 3.1 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
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
}
```
