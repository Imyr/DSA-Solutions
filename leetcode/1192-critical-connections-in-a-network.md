# 1192. Critical Connections in a Network
- [Submission](https://leetcode.com/submissions/detail/1252752680/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 131 ms | 20.8 MB |
```
use std::collections::HashMap;
impl Solution {
    fn dfs(depth: &mut usize, current: usize, previous: usize, visited: &mut Vec<bool>, depths: &mut Vec<usize>, minimums: &mut Vec<usize>, connections: &HashMap<usize, Vec<usize>>, critical: &mut Vec<Vec<i32>>) {
        *depth += 1;
        visited[current] = true;
        depths[current] = *depth;
        minimums[current] = *depth;
        for node in connections[&current].iter() {
            if !visited[*node] {
                Solution::dfs(depth, *node, current, visited, depths, minimums, connections, critical);
                minimums[current] = minimums[current].min(minimums[*node]);
            } else if *node != previous {
                minimums[current] = minimums[current].min(depths[*node]);
            }
            if depths[current] < minimums[*node] {
                critical.push(vec![current as i32, *node as i32]);
            }
        }
    }

    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut visited = vec![false; n as usize];
        let mut depths = vec![0; n as usize];
        let mut minimums = vec![0; n as usize];
        let mut critical = vec![];
        let mut depth = 0;
        let connections = connections.into_iter().fold(HashMap::new(), 
            |mut acc, x| { 
                acc.entry(x[0] as usize).or_insert(vec![]).push(x[1] as usize);
                acc.entry(x[1] as usize).or_insert(vec![]).push(x[0] as usize);
                acc
            }
        );
        Solution::dfs(&mut depth, 0, 100000, &mut visited, &mut depths, &mut minimums, &connections, &mut critical);
        critical
    }
}
```
