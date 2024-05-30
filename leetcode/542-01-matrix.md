# 542. 01 Matrix
- [Submission](https://leetcode.com/submissions/detail/1258159723/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 18 ms | 3.8 MB |
```
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut queue = std::collections::VecDeque::new();
        for i in 0..m { for j in 0..n { if mat[i][j] == 0 { 
            queue.push_back((1, i-1, j)); 
            queue.push_back((1, i+1, j)); 
            queue.push_back((1, i, j-1)); 
            queue.push_back((1, i, j+1)); 
        } else { mat[i][j] = 10001; } } }
        let mut visited = vec![vec![false; n]; m];
        while let Some((depth, i, j)) = queue.pop_front() {
            if i < 0 || j < 0 || i >= m || j >= n || mat[i][j] == 0 || visited[i][j] { continue; }
            visited[i][j] = true;
            mat[i][j] = depth.min(mat[i][j]);
            queue.push_back((depth+1, i-1, j)); 
            queue.push_back((depth+1, i+1, j)); 
            queue.push_back((depth+1, i, j-1)); 
            queue.push_back((depth+1, i, j+1)); 
        }
        mat
    }
}
```
