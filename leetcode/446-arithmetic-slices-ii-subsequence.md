# 446. Arithmetic Slices II - Subsequence
- [Submission](https://leetcode.com/submissions/detail/1139918576/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 89 ms | 15.7 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut cache = vec![HashMap::new(); nums.len()];
        let nums = nums.iter().map(|x| *x as i64).collect::<Vec<i64>>();
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i]-nums[j];
                let which = *cache[j].get(&diff).unwrap_or(&0);
                *cache[i].entry(diff).or_insert(0) += which + 1;
                sum += which;
            }
        }
        sum
    }    
}
```
- [Submission](https://leetcode.com/submissions/detail/1139918221/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 80 ms | 17.9 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut all = vec![];
        let mut cache = vec![HashMap::new(); nums.len()];
        let nums = nums.iter().map(|x| *x as i64).collect::<Vec<i64>>();
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i]-nums[j];
                let which = *cache[j].get(&diff).unwrap_or(&0);
                *cache[i].entry(diff).or_insert(0) += which + 1;
                all.push(which);
            }
        }
        all.iter().sum()
    }    
}
```
