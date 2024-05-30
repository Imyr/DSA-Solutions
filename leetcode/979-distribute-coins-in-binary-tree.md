# 979. Distribute Coins in Binary Tree
- [Submission](https://leetcode.com/submissions/detail/1261373385/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    fn dfs(mut root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut sol = vec![1, node.borrow().val, 0];
            let left = Solution::dfs(node.borrow_mut().left.take());
            let right = Solution::dfs(node.borrow_mut().right.take());
            for i in 0..3 { sol[i] += left[i] + right[i] };
            sol[2] += sol[0].abs_diff(sol[1]) as i32;
            sol
        } else { vec![0; 3] }
    }
    pub fn distribute_coins(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root)[2]
    }
}
```
