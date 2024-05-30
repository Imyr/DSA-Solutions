# 451. Sort Characters By Frequency
- [Submission](https://leetcode.com/submissions/detail/1075725684/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.4 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut sorted = String::new();
        let mut table: HashMap<char, u16> = HashMap::new();
        for c in s.chars() {
            *table.entry(c).or_insert(0)+=1;
        }
        let mut list: Vec<_> = table.iter().collect();
        list.sort_by(|(_, a), (_, b)| (*b).cmp(*a));
        for (c, i) in list {
            sorted.push_str(c.to_string().repeat(*i as usize).as_str());
        }
        sorted     
    }
}
```
