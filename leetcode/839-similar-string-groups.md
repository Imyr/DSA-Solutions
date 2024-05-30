# 839. Similar String Groups
- [Submission](https://leetcode.com/submissions/detail/1271851426/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 5 ms | 2.4 MB |
```
impl Solution {
    fn connected(graph: Vec<Vec<i8>>) -> i32 {
        let (mut res, mut visited) = (0, vec![false; graph.len()]);
        for i in 0..graph.len() {
            if !visited[i] { Self::dfs(i, &mut visited, &graph); res += 1; }
        }
        res
    }
    fn dfs(i: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<i8>>) {
        visited[i] = true;
        for j in 0..graph.len() {
            if !visited[j] && graph[i][j] == 1 { Self::dfs(j, visited, graph); }
        }
    }
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let strs: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let mut graph = vec![vec![0; n]; n];
        for i in 0..n { graph[i][i] = -1; }
        for i in 0..n {
            for j in 0..n {
                if graph[i][j] != 0 { continue; }
                let mut diff = 0;
                for idx in 0..strs[i].len() {
                    if strs[i][idx] != strs[j][idx] { 
                        if diff >= 2 { diff += 1; break; } 
                        else { diff += 1; } 
                    }
                }
                if diff <= 2 { graph[i][j] = 1; graph[j][i] = 1; } 
                else { graph[i][j] = -1; graph[j][i] = -1; }
            }
        }
        Self::connected(graph)
    }
}
```
