# 337. House Robber III
- [Submission](https://leetcode.com/submissions/detail/1272351201/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 3 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
        if let Some(mut node) = root {
            let [l0, l1] = Self::dfs(node.borrow_mut().left.take()); 
            let [r0, r1] = Self::dfs(node.borrow_mut().right.take());
            [
                l1 + node.borrow().val + r1, // [leftExcRoot, Root, rightExcRoot]
                l0.max(l1) + r0.max(r1), // [maxleft, maxRight]
            ]
        } else { [0, 0] }
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root).into_iter().max().unwrap()
    }
}
```
