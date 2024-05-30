# 136. Single Number
- [Submission](https://leetcode.com/submissions/detail/1075711426/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.3 MB |
```
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc^num)    
    }
}
```
