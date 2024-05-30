# 151. Reverse Words in a String
- [Submission](https://leetcode.com/submissions/detail/1075242757/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}
```
