# 63. Unique Paths II
- [Submission](https://leetcode.com/submissions/detail/1251610592/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        for i in 0..obstacle_grid.len() { if obstacle_grid[i][0] == 1 { dp[i][0] = 0; break; } dp[i][0] = 1; }
        for i in 0..obstacle_grid[0].len() { if obstacle_grid[0][i] == 1 { dp[0][i] = 0; break; } dp[0][i] = 1; }
        for i in 1..obstacle_grid.len() {
            for j in 1..obstacle_grid[0].len() {
                dp[i][j] = if obstacle_grid[i][j] == 1 { 0 }
                else { dp[i-1][j] + dp[i][j-1] }
            }
        }
        dp[obstacle_grid.len()-1][obstacle_grid[0].len()-1]
    }
}
```
