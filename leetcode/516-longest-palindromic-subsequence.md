# 516. Longest Palindromic Subsequence
- [Submission](https://leetcode.com/submissions/detail/1255524009/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 19 ms | 5.9 MB |
```
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![0; n+1]; n+1];
        for i in 1..n+1 {
            for j in 1..n+1 {
                if s[i-1] == s[n-1-(j-1)] { dp[i][j] = dp[i-1][j-1] + 1; } 
                else { dp[i][j] = dp[i-1][j].max(dp[i][j-1]); }
            }
        } 
        dp[n][n]
    }
}
```
