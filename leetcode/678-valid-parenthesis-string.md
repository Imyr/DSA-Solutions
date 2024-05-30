# 678. Valid Parenthesis String
- [Submission](https://leetcode.com/submissions/detail/1226191385/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
use std::collections::VecDeque;
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut asterisk: VecDeque<usize> = VecDeque::new();
        let mut open_bracket = vec![];
        for (idx, c) in s.chars().enumerate() {
            match c {
                '(' => open_bracket.push(idx),
                ')' => if open_bracket.pop().is_none() { if asterisk.pop_front().is_none() { return false; } },
                '*' => asterisk.push_back(idx),
                _ => unreachable!(),
            }
        }
        while !open_bracket.is_empty() {
            while !asterisk.is_empty() {
                if open_bracket.len() == asterisk.len() {
                    let mut flag = true;
                    for i in 0..open_bracket.len() { if open_bracket[i] > asterisk[i] { flag = false; } }
                    if flag { return true; }
                }
                if let Some(a_p) = asterisk.pop_front() {
                    if *open_bracket.last().unwrap() < a_p {
                        open_bracket.pop().unwrap();
                        break;
                    } 
                } else { return false; }
            }
            if open_bracket.len() > asterisk.len() { return false; }
        }
        true
    }
}
```
