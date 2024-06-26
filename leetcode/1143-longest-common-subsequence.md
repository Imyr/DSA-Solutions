# 1143. Longest Common Subsequence
- [Submission](https://leetcode.com/submissions/detail/1263668042/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 6 MB |
```
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let mut dp = vec![vec![0; n+1]; m+1];
        for i in 1..=m {
            for j in 1..=n {
                if text1[i-1] == text2[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
                }
            }
        }
        dp[m][n]
    }
}
```
