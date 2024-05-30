# 1254. Number of Closed Islands
- [Submission](https://leetcode.com/submissions/detail/1262894087/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut stack = vec![];
        let mut total = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 && !visited[i][j] {
                    let mut closed = true;
                    stack.push((i, j));
                    while let Some((i, j)) = stack.pop() {
                        if visited[i][j] || grid[i][j] == 1 { continue; }
                        visited[i][j] = true;
                        if i == m-1 || i == 0 || j == n-1 || j == 0 { closed = false; }
                        if i > 0 { stack.push((i-1, j)); }
                        if j > 0 { stack.push((i, j-1)); }
                        if i < m-1 { stack.push((i+1, j)); }
                        if j < n-1 { stack.push((i, j+1)); }
                    }
                    if closed { total += 1; }
                }
            }
        }
        total
    }
}
```
