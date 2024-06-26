# 450. Delete Node in a BST
- [Submission](https://leetcode.com/submissions/detail/1116755906/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 3.2 MB |
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
    fn inorder_successor(root: Rc<RefCell<TreeNode>>) -> i32 {
        if root.borrow().left.is_none() {
            root.borrow().val
        } else {
            Solution::inorder_successor(root.borrow_mut().left.clone().unwrap())
        }
    }

    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if key < node.borrow().val {
                if node.borrow().left.is_some() {
                    let nude = node.borrow_mut().left.take();
                    node.borrow_mut().left = Solution::delete_node(nude, key);
                } else {
                    return Some(node)
                }
            } else if key > node.borrow().val {
                if node.borrow().right.is_some() {
                    let nude = node.borrow_mut().right.take();
                    node.borrow_mut().right = Solution::delete_node(nude, key);
                } else {
                    return Some(node)
                }
            } else {
                if node.borrow().right.is_none() {
                    return node.borrow_mut().left.take();
                } else {
                    let mut new_node = TreeNode::new(Solution::inorder_successor(node.borrow().right.clone().unwrap()));
                    new_node.left = node.borrow_mut().left.take();
                    new_node.right = Solution::delete_node(node.borrow_mut().right.take(), new_node.val);
                    return Some(Rc::new(RefCell::new(new_node)))
                }
            }
            return Some(node)
        } else {
            return None
        }
    }
}
```
