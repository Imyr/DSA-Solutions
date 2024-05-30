# 139. Word Break
- [Submission](https://leetcode.com/submissions/detail/1254491436/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len()];
        for i in (0..s.len()).rev() {
            for word in word_dict.iter() {
                if i + word.len() == s.len() && &s[i..i+word.len()] == word.as_str() {
                    dp[i] = true;
                } else if i + word.len() < s.len() && &s[i..i+word.len()] == word.as_str() {
                    dp[i] = dp[i + word.len()];
                } 
                if dp[i] { break; }
            }
        }
        dp[0]
    }
}
```
