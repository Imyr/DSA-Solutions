# 1026. Maximum Difference Between Node and Ancestor
- [Submission](https://leetcode.com/submissions/detail/1143637247/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 3.4 MB |
```
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn max_min(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if root.is_none() {
            return (-1, -1, 0);
        }

        let val = root.as_ref().unwrap().borrow().val;
        let mut d = vec![];
        
        let (mut lmax, mut lmin, ld) = Solution::max_min(root.as_ref().unwrap().borrow().left.clone());
        let (mut rmax, mut rmin, rd) = Solution::max_min(root.as_ref().unwrap().borrow().right.clone());
        
        d.push(ld);
        d.push(rd);

        if lmax == -1 {
            lmax = val;
            lmin = val;
        }

        if rmax == -1 {
            rmax = val;
            rmin = val;
        }

        d.push((val-lmin.min(rmin)).abs());
        d.push((val-lmax.max(rmax)).abs());

        if val > lmax {
            lmax = val;
        } 
        if val < lmin {
            lmin = val;
        } 
        if val > rmax {
            rmax = val;
        } 
        if val < rmin {
            rmin = val;
        } 
        
        (lmax.max(rmax), lmin.min(rmin), d.into_iter().max().unwrap())

    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, d) = Solution::max_min(root);
        d
    }
}
```
