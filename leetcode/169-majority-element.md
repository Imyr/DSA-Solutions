# 169. Majority Element
- [Submission](https://leetcode.com/submissions/detail/1067454244/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 2.4 MB |
```
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        *counter.iter().max_by_key(|&(_, rep)| rep).unwrap().0
    }
}
```
