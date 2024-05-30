# 221. Maximal Square
- [Submission](https://leetcode.com/submissions/detail/1253906602/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 9.4 MB |
```
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = matrix.iter().map(|i| i.iter().map(|j| j.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        for i in (0..m-1).rev() {
            for j in (0..n-1).rev() {
                if dp[i][j] == 1 && dp[i][j+1] >= 1 && dp[i+1][j+1] >= 1 && dp[i+1][j+1] >= 1 {
                    dp[i][j] = dp[i+1][j+1].min(dp[i][j+1]).min(dp[i+1][j]) + 1;
                }
            }
        }
        (dp.into_iter().map(|v| v.into_iter().max().unwrap()).max().unwrap() as i32).pow(2)
    }
}
```
