# 78. Subsets
- [Submission](https://leetcode.com/submissions/detail/1263664280/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..2i32.pow(nums.len() as u32)).map(|b| {
            (0..nums.len()).fold(vec![], |mut acc, i| { if b&1<<i>0 { acc.push(nums[i]) }; acc }
        )}).collect()
    }
}
```
