# 1061. Lexicographically Smallest Equivalent String
- [Submission](https://leetcode.com/submissions/detail/1270389485/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let n = s1.len();
        let (s1, s2) = (s1.chars().collect::<Vec<char>>(), s2.chars().collect::<Vec<char>>());
        let mut map = ('a'..='z').fold(HashMap::new(), |mut acc, c| { 
            acc.entry(c).or_insert(HashSet::new()).insert(c);
            acc 
        });
        for i in 0..n {
            if let Some(mut k) = map.get_mut(&s1[i]) { k.insert(s2[i]); }
            if let Some(mut k) = map.get_mut(&s2[i]) { k.insert(s1[i]); }
        }
        let mut char_map = HashMap::new();
        for c in 'a'..='z' {
            if char_map.get(&c).is_none() {
                let (mut visited, mut min) = (HashSet::new(), c);
                let mut stack = vec![c];
                while let Some(c1) = stack.pop() {
                    if !visited.insert(c1) { continue; }
                    if c1 < min { min = c1; }
                    for c2 in map[&c1].iter() { if !visited.contains(c2) { stack.push(*c2); } }
                }
                for x in visited.into_iter() {
                    char_map.insert(x, min);
                }
            }

        }
        base_str.chars().into_iter().map(|x| if let Some(c) = char_map.get(&x) { *c } else { x }).collect()
    }
}
```
