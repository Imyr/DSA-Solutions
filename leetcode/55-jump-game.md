# 55. Jump Game
- [Submission](https://leetcode.com/submissions/detail/1250757266/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 95 ms | 2.2 MB |
```
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len()];
        dp[nums.len()-1] = true;
        for i in (0..nums.len()-1).rev() {
            for j in i+1..=i+(nums[i] as usize) {
                if dp[j] { dp[i] = true; break; }
            }
        }
        dp[0]
    }
}
```
