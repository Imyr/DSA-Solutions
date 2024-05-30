# 3. Longest Substring Without Repeating Characters
- [Submission](https://leetcode.com/submissions/detail/935373996/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 2.1 MB |
```
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        let mut list = s.chars();
        let mut word = list.next().unwrap().to_string();
        let mut len = 0;
        for c in list {
            if word.contains(c) {
                let (_, word2) = word.split_once(c).unwrap();
                if word2 == "" {
                    word = c.to_string();
                }
                else {
                    word = word2.to_owned() + &c.to_string();
                }
            } else {
                word = format!("{}{}", word, c);
            }
            if word.len() > len {
                len = word.len();
            } 
        }
        len as i32
    }
}
```
