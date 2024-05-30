# 106. Construct Binary Tree from Inorder and Postorder Traversal
- [Submission](https://leetcode.com/submissions/detail/1117102500/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 38.1 MB |
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None
        }
        let idx = inorder.iter().position(|i| i == postorder.last().unwrap()).unwrap();
        let (left_inorder, right_inorder) = (inorder[0..idx].to_vec(), inorder[idx+1..inorder.len()].to_vec());
        let (left_postorder, right_postorder) = (postorder[0..left_inorder.len()].to_vec(), postorder[left_inorder.len()..postorder.len()-1].to_vec());
        let mut node = TreeNode::new(*postorder.last().unwrap());
        node.left = Solution::build_tree(left_inorder, left_postorder);
        node.right = Solution::build_tree(right_inorder, right_postorder);
        
        Some(Rc::new(RefCell::new(node)))
    }
}
```
