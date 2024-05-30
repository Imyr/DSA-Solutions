# 934. Shortest Bridge
- [Submission](https://leetcode.com/submissions/detail/1254465033/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 8 ms | 2.4 MB |
```
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut island1 = vec![vec![false; n]; n];
        let mut island2 = vec![vec![0; n]; n];
        let mut visited = vec![vec![false; n]; n];
        let mut queue = std::collections::VecDeque::new();
        let mut flag = true;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && !island1[i][j] && island2[i][j] == 0  {
                    queue.push_back((i, j));
                    while let Some((i, j)) = queue.pop_front() {
                        if visited[i][j] || grid[i][j] == 0 { continue; }
                        visited[i][j] = true;
                        if flag { island1[i][j] = true; } else { island2[i][j] = 1; }
                        if i > 0 { queue.push_back((i-1, j)); }
                        if j > 0 { queue.push_back((i, j-1)); }
                        if i < n-1 { queue.push_back((i+1, j)); }
                        if j < n-1 { queue.push_back((i, j+1)); }
                    }
                    flag = false;
                }
            }
        }
        let mut queue = std::collections::VecDeque::new();
        visited = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if island1[i][j] {
                    queue.push_back((-1, i, j));
                }
            }
        }
        while let Some((depth, i, j)) = queue.pop_front() {
            if visited[i][j] { continue; }
            if island2[i][j] == 1 { return depth; }
            visited[i][j] = true;
            island2[i][j] = depth;
            if i > 0 { queue.push_back((depth+1, i-1, j)); }
            if j > 0 { queue.push_back((depth+1, i, j-1)); }
            if i < n-1 { queue.push_back((depth+1, i+1, j)); }
            if j < n-1 { queue.push_back((depth+1, i, j+1)); }
        }
        0
    }
}
```
