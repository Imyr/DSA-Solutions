# 2385. Amount of Time for Binary Tree to Be Infected
- [Submission](https://leetcode.com/submissions/detail/1142611242/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 27 ms | 21.1 MB |
```
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn check_subtree(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> (i32, i32) {
        if let Some(node) = root {
            let left = Solution::check_subtree(node.borrow_mut().left.take(), start);
            let right = Solution::check_subtree(node.borrow_mut().right.take(), start);
            if node.borrow().val == start {
                (0, left.1.max(right.1) + 1)
            } else if left.0 != -1 {
                (left.0 + 1, left.1.max(right.1 + left.0 + 2))
            } else if right.0 != -1 {
                (right.0 + 1, right.1.max(left.1 + right.0 + 2))
            } else {
                (-1, left.1.max(right.1) + 1)
            }
        } else {
            (-1, -1)
        }
    }

    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Solution::check_subtree(root, start).1
    }
}
```
