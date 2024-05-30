# 2997. Minimum Number of Operations to Make Array XOR Equal to K
- [Submission](https://leetcode.com/submissions/detail/1245231684/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 10 ms | 3.4 MB |
```
    impl Solution {
        pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
            nums.iter().fold(k, |acc, x| acc^x).count_ones() as i32
        }
    }
```
