# 1615. Maximal Network Rank
- [Submission](https://leetcode.com/submissions/detail/1266876301/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 10 ms | 2.4 MB |
```
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = roads.into_iter().fold(vec![std::collections::HashSet::new(); n as usize], |mut acc, x| {
            acc[x[0] as usize].insert(x[1]); acc[x[1] as usize].insert(x[0]); 
            acc
        });
        (0..n).map(|i| (i+1..n).map(|j| {
            graph[i].len() + graph[j].len() - if graph[i].contains(&(j as i32)) { 1 } else { 0 }
        }).max().unwrap_or_else(|| 0)).max().unwrap() as _
    }
}
```
