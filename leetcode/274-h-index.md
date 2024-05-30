# 274. H-Index
- [Submission](https://leetcode.com/submissions/detail/1251629568/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.3 MB |
```
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let mut h = 0;
        for i in (0..citations.len()).rev() {
            if citations[i] > h { h += 1; } 
            else { break; }
        }
        h
    }
}
```
