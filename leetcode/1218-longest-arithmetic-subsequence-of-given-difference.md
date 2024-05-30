# 1218. Longest Arithmetic Subsequence of Given Difference
- [Submission](https://leetcode.com/submissions/detail/1260744527/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 21 ms | 3.1 MB |
```
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let n = arr.len();
        let mut dp = std::collections::HashMap::new();
        for i in 0..n {
            if let Some(c) = dp.get(&(arr[i]-difference)) { dp.insert(arr[i], c+1); } 
            else { dp.insert(arr[i], 1); }
        }
        dp.into_iter().map(|(_, x)| x).max().unwrap()
    }
}
```
