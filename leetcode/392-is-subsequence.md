# 392. Is Subsequence
- [Submission](https://leetcode.com/submissions/detail/1056321909/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut check = match s.next() {
            Some(c) => c,
            None => return true,
        };

        for c in t.chars() {
            if c==check {
                check = match s.next() {
                    Some(c) => c,
                    None => return true,
                }
            }
        }
        false
    }
}
```
