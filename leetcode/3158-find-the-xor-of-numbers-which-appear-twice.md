# 3158. Find the XOR of Numbers Which Appear Twice
- [Submission](https://leetcode.com/submissions/detail/1267532337/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in nums.into_iter() {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut xor = 0;
        for (x, c) in map.into_iter() {
            if c == 2 {
                xor ^= x;
            }
        }
        xor
    }
}
```
