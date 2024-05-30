# 200. Number of Islands
- [Submission](https://leetcode.com/submissions/detail/1261078834/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 9 MB |
```
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        let mut total = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && !visited[i][j] {
                    total += 1;
                    queue.push_back((i, j));
                    while let Some((i, j)) = queue.pop_front() {
                        if visited[i][j] || grid[i][j] == '0' { continue; }
                        visited[i][j] = true;
                        if i > 0 { queue.push_back((i-1, j)); }
                        if j > 0 { queue.push_back((i, j-1)); }
                        if i < m-1 { queue.push_back((i+1, j)); }
                        if j < n-1 { queue.push_back((i, j+1)); }
                    }
                }
            }
        }
        total
    }
}
```
