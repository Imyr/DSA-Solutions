# 2482. Difference Between Ones and Zeros in Row and Column
- [Submission](https://leetcode.com/submissions/detail/1119520563/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 66 ms | 13.4 MB |
```
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (lc, lr) = (grid[0].len(), grid.len());
        let (mut columns, mut rows) = (vec![-1; lc], vec![-1; lr]);
        let mut diff = vec![vec![-1; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let ones_row;
                let ones_col;
                if rows[i] != -1 {
                    ones_row = rows[i];
                } else {
                    rows[i] = grid[i].iter().filter(|&&x| x == 1).count() as i32;
                    ones_row = rows[i];
                }
                if columns[j] != -1 {
                    ones_col = columns[j];
                } else {
                    columns[j] = grid.iter().map(|row| row[j]).filter(|&x| x == 1).count() as i32;
                    ones_col = columns[j];
                }
                diff[i][j] = ones_row + ones_col - (lr as i32 - ones_row) - (lc as i32 - ones_col);
            }
        }
        diff
    }
}
```
