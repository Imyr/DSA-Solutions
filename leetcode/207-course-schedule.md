# 207. Course Schedule
- [Submission](https://leetcode.com/submissions/detail/1272325292/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.8 MB |
```
use std::collections::HashSet;
impl Solution {
    fn dfs(i: usize, visiting: &mut Vec<bool>, graph: &mut Vec<HashSet<usize>>) -> bool {
        if visiting[i] { return false; } visiting[i] = true;
        let pr = graph[i].iter().map(|x| *x).collect::<Vec<_>>();
        for j in pr { if !Self::dfs(j, visiting, graph) { return false; } }
        graph[i].clear(); visiting[i] = false;
        true
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![HashSet::new(); num_courses as usize];
        for x in prerequisites { graph[x[0] as usize].insert(x[1] as usize); }
        let mut visiting = vec![false; num_courses as usize];
        for i in 0..graph.len() { if !Self::dfs(i, &mut visiting, &mut graph) { return false; } }
        true
    }
}
```
