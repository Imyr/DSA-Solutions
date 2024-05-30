# 1325. Delete Leaves With a Given Value
- [Submission](https://leetcode.com/submissions/detail/1260320620/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.4 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(mut node) = root {
            let left = Self::remove_leaf_nodes(node.borrow_mut().left.take(), target);
            let right = Self::remove_leaf_nodes(node.borrow_mut().right.take(), target);
            if left.is_none() && right.is_none() && node.borrow().val == target {
                return None
            }
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
            Some(node)
        } else { None }
    }
}
```
