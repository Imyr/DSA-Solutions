# 872. Leaf-Similar Trees
- [Submission](https://leetcode.com/submissions/detail/1141543194/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn leavise(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut leafs = vec![];
        if root.is_none() {
            return vec![];
        }
        let node = root.as_ref().unwrap().borrow();
        leafs.append(&mut Solution::leavise(&node.left));
        if node.left.is_none() && node.right.is_none() {
            leafs.push(node.val);
        }
        leafs.append(&mut Solution::leavise(&node.right));
        leafs
    }
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::leavise(&root1) == Solution::leavise(&root2)
    }
}
```
