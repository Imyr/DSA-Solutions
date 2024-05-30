# 863. All Nodes Distance K in Binary Tree
- [Submission](https://leetcode.com/submissions/detail/1259788071/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
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
use std::collections::{HashSet, HashMap, VecDeque};
impl Solution {
    fn tree_to_graph(root: Rc<RefCell<TreeNode>>) -> HashMap<i32, Vec<i32>> {
      let mut graph = HashMap::new();
      let mut stack = vec![(None, root)];
      while let Some((pval, node)) = stack.pop() {
        let cval = node.borrow().val;
        if let Some(left) = node.borrow_mut().left.take() {
          stack.push((Some(cval), left));
        };
        if let Some(right) = node.borrow_mut().right.take() {
          stack.push((Some(cval), right));
        };
        if pval.is_some() {
          let pval = pval.unwrap();
            graph.entry(pval).or_insert(vec![]).push(cval);
          graph.entry(cval).or_insert(vec![]).push(pval);
        }
      }
      graph
  }
  
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
      let graph = Solution::tree_to_graph(root.unwrap());
      let mut visited = HashSet::new();
      let mut queue = VecDeque::new();
      let node = target.unwrap();
      queue.push_back((0, node.borrow().val));
      let mut res = vec![];
      while let Some((depth, val)) = queue.pop_front() {
        if !visited.insert(val) { continue; }
        if depth == k {
          res.push(val);
          continue;
        }
        if graph.get(&val).is_none() { continue; } 
        for &next in graph[&val].iter() {
          queue.push_back((depth+1, next));
        }
      }
      res
    }
}
```
