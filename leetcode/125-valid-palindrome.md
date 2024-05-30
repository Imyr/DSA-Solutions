# 125. Valid Palindrome
- [Submission](https://leetcode.com/submissions/detail/1079432493/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_lowercase().to_string()).collect();
        s.chars().eq(s.chars().rev())
    }   
}
```
