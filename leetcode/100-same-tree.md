# 100. Same Tree
- [Submission](https://leetcode.com/submissions/detail/1264353515/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val 
                &&
                Self::is_same_tree(p.borrow_mut().left.take(), q.borrow_mut().left.take())
                &&
                Self::is_same_tree(p.borrow_mut().right.take(), q.borrow_mut().right.take())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
```
