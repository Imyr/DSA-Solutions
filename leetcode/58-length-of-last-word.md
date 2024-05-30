# 58. Length of Last Word
- [Submission](https://leetcode.com/submissions/detail/1219875851/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split_ascii_whitespace().into_iter().last().unwrap().len() as i32
    }
}
```
