# 20. Valid Parentheses
- [Submission](https://leetcode.com/submissions/detail/1020020542/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut hmm = vec![];
        for c in s.chars() {
            match c {
                '(' => hmm.push(c),
                ')' => {
                    if hmm.pop()!=Some('(') {
                        return false;
                    }
                },
                '{' => hmm.push(c),
                '}' => {
                    if hmm.pop()!=Some('{') {
                        return false;
                    }
                },
                '[' => hmm.push(c),
                ']' => {
                    if hmm.pop()!=Some('[') {
                        return false;
                    }
                },
                _ => continue,
            }
        }
        hmm.len()==0
    }
}
```
