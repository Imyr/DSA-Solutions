# 1863. Sum of All Subset XOR Totals
- [Submission](https://leetcode.com/submissions/detail/1263306592/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2 MB |
```
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        (0..2i32.pow(nums.len() as u32)).map(|b| {
            (0..nums.len()).fold(0, |mut acc, i| { if b&1<<i>0 { acc^=nums[i] }; acc }
        )}).sum()
    }
}
```
