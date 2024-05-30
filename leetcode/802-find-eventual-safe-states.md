# 802. Find Eventual Safe States
- [Submission](https://leetcode.com/submissions/detail/1248507739/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 3 MB |
```
impl Solution {
    fn safe(i: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, tv: &mut Vec<bool>, bad: &mut Vec<bool>) -> bool {
        visited[i] = true;
        tv[i] = true;
        bad[i] = true;
        for j in graph[i].iter().map(|x| *x as usize) {
            if !visited[j] {
                if !Solution::safe(j, graph, visited, tv, bad) {
                    bad[j] = true;
                    return false;
                }
            } else if tv[j] {
                bad[j] = true;
                return false;   
            }
        }
        bad[i] = false;
        tv[i] = false;
        return true;
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![false; graph.len()];
        let mut bad = vec![false; graph.len()];
        let mut tv = vec![false; graph.len()];

        for i in 0..graph.len() {
            if !visited[i] {
                Solution::safe(i, &graph, &mut visited, &mut tv, &mut bad);
            }
        }
        bad.into_iter().enumerate().filter(|(_, b)| !b).map(|(x, _)| x as i32).collect()
    }
}
```
