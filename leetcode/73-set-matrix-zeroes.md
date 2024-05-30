# 73. Set Matrix Zeroes
- [Submission](https://leetcode.com/submissions/detail/1260325491/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 2.4 MB |
```
use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut ri, mut rj) = (HashSet::new(), HashSet::new());
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 { ri.insert(i); rj.insert(j); }
            }
        }
        for i in ri.into_iter() {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
        for j in rj.into_iter() {
            for i in 0..m {
                matrix[i][j] = 0;
            }
        }
    }
}
```
