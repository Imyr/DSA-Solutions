# 1027. Longest Arithmetic Subsequence
- [Submission](https://leetcode.com/submissions/detail/1261150692/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 277 ms | 5.6 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = 2;
        let mut cache = vec![HashMap::new(); n];
        for i in 0..n {
            for j in 0..i {
                let mut count;
                if let Some(&c) = cache[j].get(&(nums[i]-nums[j])) {
                    count = c+1;
                } else { count = 2; }
                max = max.max(count);
                cache[i].insert(nums[i]-nums[j], count);
            }
        }
        max
    }
}
```
