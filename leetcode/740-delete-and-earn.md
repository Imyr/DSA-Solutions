# 740. Delete and Earn
- [Submission](https://leetcode.com/submissions/detail/1249353660/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.3 MB |
```
impl Solution {
    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
        let cmap = nums.iter().map(|y| *y).fold(std::collections::HashMap::new(), 
            |mut acc, x| { *acc.entry(x).or_insert(0)+=1; acc });
        nums.sort_unstable();  
        nums.dedup(); 
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0]*cmap[&nums[0]];
        if dp.len() > 1 {
            dp[1] = if nums[1]-1==nums[0] { dp[0].max(nums[1]*cmap[&nums[1]]) } else { dp[0]+nums[1]*cmap[&nums[1]] };
        }
        for i in 2..nums.len() {
            dp[i] = if nums[i]-1==nums[i-1] { dp[i-1].max(dp[i-2]+nums[i]*cmap[&nums[i]]) } else { dp[i-1]+nums[i]*cmap[&nums[i]] };
        }
        dp[dp.len()-1]
    }
}
```
