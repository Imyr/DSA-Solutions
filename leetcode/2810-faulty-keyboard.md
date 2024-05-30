# 2810. Faulty Keyboard
- [Submission](https://leetcode.com/submissions/detail/1064129957/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut hmm = String::new();
        for c in s.chars() {
            if c == 'i' {
                hmm = hmm.chars().rev().collect();
            } else {
                hmm += c.to_string().as_str();
            }
        }
        hmm
    }
}
```
