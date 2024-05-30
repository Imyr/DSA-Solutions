# 45. Jump Game II
- [Submission](https://leetcode.com/submissions/detail/1250778543/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 17 ms | 2.3 MB |
```
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; nums.len()];
        dp[nums.len()-1] = 0;
        for i in (0..nums.len()-1).rev() {
            dp[i] = if let Some(min) = dp[i+1..=(i+(nums[i] as usize)).min(nums.len()-1)].iter().min() { 
                if *min==i32::MAX {i32::MAX}
                else { *min + 1 }
                } 
            else { i32::MAX };
        }
        dp[0]
    }
}
```
