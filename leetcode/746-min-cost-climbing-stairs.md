# 746. Min Cost Climbing Stairs
- [Submission](https://leetcode.com/submissions/detail/1225161833/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![cost[0], cost[1]];
        for i in 2..cost.len() { dp.push(cost[i] + dp[i-1].min(dp[i-2])); }
        dp[dp.len()-1].min(dp[dp.len()-2])
    }
}
```
