# 1662. Check If Two String Arrays are Equivalent
- [Submission](https://leetcode.com/submissions/detail/1117449294/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}
```
