# 2373. Largest Local Values in a Matrix
- [Submission](https://leetcode.com/submissions/detail/1256050518/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut values = vec![vec![0; n-2]; n-2];
        for i in 1..n-1 {
            for j in 1..n-1 {
                let mut max = 0;
                for k in i-1..=i+1 {
                    for l in j-1..=j+1 {
                        max = max.max(grid[k][l]);
                    }
                }
                values[i-1][j-1] = max;
            }
        }
        values
    }
}
```
