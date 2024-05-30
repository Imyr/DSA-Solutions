# 242. Valid Anagram
- [Submission](https://leetcode.com/submissions/detail/1120815190/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 10 ms | 2.6 MB |
```
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (mut ss, mut st) = (s.chars().collect::<Vec<char>>(), t.chars().collect::<Vec<char>>());
        ss.sort();
        st.sort();
        ss == st
    }
}
```
