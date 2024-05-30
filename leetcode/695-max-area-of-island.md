# 695. Max Area of Island
- [Submission](https://leetcode.com/submissions/detail/1263963844/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.4 MB |
```
impl Solution {
    fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
        if i<0 || j<0 || i>=grid.len() || j>=grid[0].len() || grid[i][j] == 0 || visited[i][j] { return 0; }
        visited[i][j] = true;
        1 + Solution::dfs(i+1, j, grid, visited)
        + Solution::dfs(i-1, j, grid, visited)
        + Solution::dfs(i, j+1, grid, visited)
        + Solution::dfs(i, j-1, grid, visited)
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && !visited[i][j] {
                    max = max.max(Solution::dfs(i, j, &grid, &mut visited));
                }
            }
        }
        max
    }
}
```
