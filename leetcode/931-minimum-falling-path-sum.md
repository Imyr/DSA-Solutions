# 931. Minimum Falling Path Sum
- [Submission](https://leetcode.com/submissions/detail/1150432263/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.4 MB |
```
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; matrix.len()]; matrix.len()];
        
        for j in 0..matrix.len() {
            dp[0][j] = matrix[0][j];
        }

        for i in 1..matrix.len() {
            for j in 0..matrix.len() {
                dp[i][j] = matrix[i][j] + dp[i-1][j].min(if j==0 {i32::MAX} else {dp[i-1][j-1]}).min(if j==matrix.len()-1 {i32::MAX} else {dp[i-1][j+1]});
            }
        }
        *dp[matrix.len()-1].iter().min().unwrap()
    }
}
```
