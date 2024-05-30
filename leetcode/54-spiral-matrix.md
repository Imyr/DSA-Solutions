# 54. Spiral Matrix
- [Submission](https://leetcode.com/submissions/detail/1259046317/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (m, n);
        
        let mut run_i = true;
        let mut incrm = true;

        let (mut i, mut j) = (0, 0);
        let mut spiral = vec![];
        while spiral.len() != m*n {
            while j < y && spiral.len() != m*n {
                spiral.push(matrix[i][j]);
                j += 1;
            } j -= 1; i += 1;
            while i < x && spiral.len() != m*n {
                spiral.push(matrix[i][j]);
                i += 1;
            } i -= 1;  j -= 1; x -= 1;
            while j > n - y && spiral.len() != m*n{
                spiral.push(matrix[i][j]);
                j -= 1;
            } y -= 1;
            while i > m - x  && spiral.len() != m*n{
                spiral.push(matrix[i][j]);
                i -= 1;
            } 
        }
        spiral
    }
}
```
