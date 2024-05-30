# 938. Range Sum of BST
- [Submission](https://leetcode.com/submissions/detail/1140307780/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 10 ms | 4.2 MB |
```
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(mut root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut sum = 0;
    let mut stack = vec![];
    loop {
        if root.is_some() {
            let node = root.take().unwrap();
            root = node.borrow_mut().left.take();
            stack.push(node);
        } else if !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node.borrow_mut().right.take() {
                Some(right) => stack.push(right),
                None => {},
            };
            if node.borrow().left.is_some() {
                root = node.borrow_mut().left.take();
                stack.push(node);
            } else {
                if node.borrow().val > high {
                    break;
                } else if node.borrow().val >= low {
                    sum += node.borrow().val;
                }
            }
        } else {
            break;
        }
    }
    sum
    }
}
```
