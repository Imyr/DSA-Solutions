# 861. Score After Flipping Matrix
- [Submission](https://leetcode.com/submissions/detail/1256853035/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n { grid[i][j] ^= 1; }
            }
        }
        for j in 1..n {
            if (0..m).map(|i| grid[i][j]).sum::<i32>() <= (m/2) as i32 {
                for i in 0..m { grid[i][j] ^= 1; }
            }
        }
        grid.into_iter().map(|r| r.into_iter().fold(0, 
            |mut acc, x| {
                acc <<= 1; acc += x;  acc
            }
        )).sum::<i32>()
    }
}
```
