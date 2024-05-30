# 72. Edit Distance
- [Submission](https://leetcode.com/submissions/detail/1256303630/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.9 MB |
```
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n1 = word1.len();
        let n2 = word2.len();
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![0; n2+1]; n1+1];
        for i in 1..n1+1 { dp[i][0] = i as i32; }
        for j in 1..n2+1 { dp[0][j] = j as i32; }
        for i in 1..n1+1 {
            for j in 1..n2+1 {
                if word1[i-1] == word2[j-1] { dp[i][j] = dp[i-1][j-1]; } 
                else { dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]) + 1; }
            }
        } 
        dp[n1][n2]
    }
}
```
