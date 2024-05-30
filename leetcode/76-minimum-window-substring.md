# 76. Minimum Window Substring
- [Submission](https://leetcode.com/submissions/detail/1258144719/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 22 ms | 2.5 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut t = t.chars().into_iter().fold(HashMap::new(), 
            |mut acc, c| { *acc.entry(c).or_insert(0) += 1; acc }
        );
        let (mut min_i, mut min_j) = (0, 100000);
        let (mut i, mut j) = (0, 0);
        while j < s.len() {
            if let Some(mut n) = t.get_mut(&s[j]) {
                *n -= 1;
            } else { j += 1; continue; }
            while t.iter().map(|(_, n)| n).filter(|&&n| n > 0).count() == 0 {
                while i < s.len() {
                    if j - i < min_j - min_i {
                        (min_i, min_j) = (i, j);
                    }
                    if let Some(mut n) = t.get_mut(&s[i]) {
                        *n += 1;
                        i += 1;
                        break;
                    } else { i += 1; }
                }
            } 
            j += 1;
        }
        if min_j != 100000 { s[min_i..=min_j].iter().collect() } else { "".to_string() }
    }
}
```
