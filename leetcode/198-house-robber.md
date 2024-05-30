# 198. House Robber
- [Submission](https://leetcode.com/submissions/detail/1248277665/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0] }
        else if nums.len() == 2 { return nums[1].max(nums[0]) }
        else {
            *(3..nums.len()).fold(vec![nums[0], nums[1], nums[2]+nums[0]], |mut acc, x| {
                acc.push(nums[x] + acc[x-2].max(acc[x-3]));
                acc
            }).windows(2).last().unwrap().into_iter().max().unwrap() as _
        }
    }
}
```
