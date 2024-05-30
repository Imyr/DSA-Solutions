# 2000. Reverse Prefix of Word
- [Submission](https://leetcode.com/submissions/detail/1246727532/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(idx) = word.find(ch) {
            word[0..=idx].chars().rev().collect::<String>() + &word[idx+1..]
        } else {
            word
        }
    }
}
```
