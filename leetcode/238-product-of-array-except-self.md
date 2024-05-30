# 238. Product of Array Except Self
- [Submission](https://leetcode.com/submissions/detail/1252574087/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 9 ms | 3.3 MB |
```
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut suffix, mut product) = (1, vec![1; nums.len()]);
        for i in 1..nums.len() {
            product[i] = nums[i-1]*product[i-1];
        }
        for i in (0..nums.len()-1).rev() {
            suffix *= nums[i+1];
            product[i] *= suffix;
        }
        product
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1252557883/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 11 ms | 3.6 MB |
```
use std::collections::VecDeque;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix = nums.iter().map(|x| *x).fold(vec![], |mut acc, x| {
            acc.push(*acc.last().unwrap_or(&1)*x);
            acc
        });
        let suffix = nums.iter().rev().map(|x| *x).fold(VecDeque::new(), |mut acc: VecDeque<i32>, x| {
            acc.push_front(*acc.front().unwrap_or(&1)*x);
            acc
        });
        (0..nums.len()).map(|i| 
            (if i != 0 { prefix[i-1] } else { 1 }) * (if i != nums.len()-1 { suffix[i+1] } else { 1 }) 
        ).collect()
    }
}
```
