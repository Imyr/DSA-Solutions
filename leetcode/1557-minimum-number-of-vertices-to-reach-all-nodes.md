# 1557. Minimum Number of Vertices to Reach All Nodes
- [Submission](https://leetcode.com/submissions/detail/1265594023/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 32 ms | 8.8 MB |
```
use std::collections::HashSet;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut present = vec![true; n as usize];
        for x in edges { present[x[1] as usize] = false; }
        present.into_iter().enumerate().filter(|y| y.1).map(|y| y.0 as i32).collect()
    }
}
```
