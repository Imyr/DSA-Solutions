# 48. Rotate Image
- [Submission](https://leetcode.com/submissions/detail/1259596592/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i+1..n {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]); 
            }
        }
        for i in 0..n {
            for j in 0..n/2 {
                (matrix[i][j], matrix[i][n-j-1]) = (matrix[i][n-j-1], matrix[i][j]); 
            }
        }  
    }
}
```
