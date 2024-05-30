# 1422. Maximum Score After Splitting a String
- [Submission](https://leetcode.com/submissions/detail/1125850479/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut zeroes: HashMap<usize, i32> = HashMap::new();
        let mut ones: HashMap<usize, i32> = HashMap::new();
        let mut zero = 0;
        for i in 0..s.len() {
            if s[i] == '0' {
                zero += 1;
            }
            zeroes.insert(i, zero);
        }
        let mut one = 0;
        for i in (0..s.len()).rev() {
            ones.insert(i, one);
            if s[i] == '1' {
                one += 1;
            }
        }
        let mut max = i32::MIN;
        for i in 0..s.len()-1 {
            if zeroes[&i]+ones[&i] > max {
                max = zeroes[&i]+ones[&i];
            }
        }
        max
    }
}
```
