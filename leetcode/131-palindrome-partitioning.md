# 131. Palindrome Partitioning
- [Submission](https://leetcode.com/submissions/detail/1264877663/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 67 ms | 21 MB |
```
use std::collections::HashSet;
impl Solution {
    fn merge(i: usize, mut s: Vec<String>, pdr: &mut Vec<Vec<String>>) {
        if i == s.len()-1 { 
            if s.iter().map(|x| x.chars().eq(x.chars().rev())).filter(|&y| y).count() == s.len() {
                pdr.push(s); 
            } 
            return; 
        }
        Self::merge(i+1, s.clone(), pdr);
        let new = s[i+1].clone();
        s[i] += &new; s.remove(i+1); 
        Self::merge(i, s, pdr);
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let mut pdr = vec![];
        Self::merge(0, s, &mut pdr);
        pdr
    }
}
```
