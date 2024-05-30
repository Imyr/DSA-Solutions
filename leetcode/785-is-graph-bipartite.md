# 785. Is Graph Bipartite?
- [Submission](https://leetcode.com/submissions/detail/1267863217/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 2.5 MB |
```
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let (mut true_set, mut false_set) = (HashSet::new(), HashSet::new());
        let mut queue = VecDeque::new();
        for i in 0..graph.len() {
            if true_set.contains(&i) || false_set.contains(&i) { continue; }
            queue.push_back((true, i));
            while let Some((set, node)) = queue.pop_front() {
                if set { if false_set.contains(&node) { return false; }; if !true_set.insert(node) { continue; } }
                else { if true_set.contains(&node) { return false; } if !false_set.insert(node) { continue; }  }
                for &next in graph[node].iter() {
                    queue.push_back((!set, next as usize));
                }
            }
        }
        true
    }
}
```
