# 189. Rotate Array
- [Submission](https://leetcode.com/submissions/detail/1249006848/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 3.4 MB |
```
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        *nums = nums[nums.len()-(k as usize % nums.len())..nums.len()].iter()
        .chain(nums[0..nums.len()-(k as usize % nums.len())].iter())
        .map(|x| *x).collect::<Vec<_>>();
    }
}
```
