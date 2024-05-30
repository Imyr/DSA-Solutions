# 2784. Check if Array is Good
- [Submission](https://leetcode.com/submissions/detail/1079342748/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        nums.sort();
        if nums.len() != *nums.last().unwrap() as usize + 1 {
            return false
        }
        for i in 0..nums.len()-1 {
            if nums[i] != i as i32 + 1 {
                return false
            }
        }
        true 
    }
}
```
