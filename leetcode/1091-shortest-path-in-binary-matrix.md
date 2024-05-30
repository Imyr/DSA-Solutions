# 1091. Shortest Path in Binary Matrix
- [Submission](https://leetcode.com/submissions/detail/1258984925/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 2.2 MB |
```
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 0));
        while let Some((depth, i, j)) = queue.pop_front() {
            if i < 0 || j < 0 || i >= m || j >= n || grid[i][j] == 1 || visited[i][j] { continue; }
            visited[i][j] = true;
            if i == m-1 && j == n-1 { return depth+1; }
            queue.push_back((depth+1, i-1, j)); 
            queue.push_back((depth+1, i+1, j)); 
            queue.push_back((depth+1, i, j-1)); 
            queue.push_back((depth+1, i, j+1)); 
            queue.push_back((depth+1, i-1, j+1)); 
            queue.push_back((depth+1, i+1, j+1)); 
            queue.push_back((depth+1, i-1, j-1)); 
            queue.push_back((depth+1, i+1, j-1)); 
        }
        -1
    }
}
```
