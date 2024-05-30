# 95. Unique Binary Search Trees II
- [Submission](https://leetcode.com/submissions/detail/1271799464/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.8 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn recurse(l: i32, h: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l > h { return vec![None]; }
        (l..=h).fold(vec![], |mut acc, m| {
            for lt in Self::recurse(l, m-1) {
                for rt in Self::recurse(m+1, h) {
                    acc.push(Some(Rc::new(RefCell::new(
                        // [1..r] left + r + [r+1..=n] right
                        TreeNode { val: m, left: lt.clone(), right: rt }
                    ))))
                }
            }
            acc
        })
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::recurse(1, n)
    }
}
```
