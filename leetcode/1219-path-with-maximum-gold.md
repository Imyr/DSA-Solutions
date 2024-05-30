# 1219. Path with Maximum Gold
- [Submission](https://leetcode.com/submissions/detail/1257807831/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1540 ms | 2 MB |
```
impl Solution {
    fn max_gold(i: usize, j: usize, grid: &Vec<Vec<i32>>, mut visited: Vec<Vec<bool>>) -> i32 {
        if i < 0 || i >= grid.len() || j < 0 || j >= grid[0].len() || grid[i][j] == 0 || visited[i][j] { return 0; }
        visited[i][j] = true;
        
        grid[i][j] + Solution::max_gold(i-1, j, &grid, visited.clone())
        .max(Solution::max_gold(i+1, j, &grid, visited.clone()))
        .max(Solution::max_gold(i, j-1, &grid, visited.clone()))
        .max(Solution::max_gold(i, j+1, &grid, visited.clone()))

    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut max = 0;
        for i in 0..m {
            for j in 0..n { max = Solution::max_gold(i, j, &grid, vec![vec![false; n]; m]).max(max) }
        }
        max
    }
}
```
