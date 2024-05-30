# 1137. N-th Tribonacci Number
- [Submission](https://leetcode.com/submissions/detail/1225101778/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![0, 1, 1];
        for i in 3..=n as usize { dp.push(dp[i-1]+dp[i-2]+dp[i-3]); }
        dp[n as usize]
    }
}
```
