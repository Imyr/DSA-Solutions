# 105. Construct Binary Tree from Preorder and Inorder Traversal
- [Submission](https://leetcode.com/submissions/detail/1117103403/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 38 MB |
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None
        }

        let idx = inorder.iter().position(|i| i == preorder.first().unwrap()).unwrap();
        let (left_inorder, right_inorder) = (inorder[0..idx].to_vec(), inorder[idx+1..inorder.len()].to_vec());
        let (left_preorder, right_preorder) = (preorder[1..=left_inorder.len()].to_vec(), preorder[left_inorder.len()+1..preorder.len()].to_vec());

        let mut node = TreeNode::new(*preorder.first().unwrap());
        node.left = Solution::build_tree(left_preorder, left_inorder);
        node.right = Solution::build_tree(right_preorder, right_inorder);
        
        Some(Rc::new(RefCell::new(node)))
    }
}
```
