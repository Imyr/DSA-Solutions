# 1207. Unique Number of Occurrences
- [Submission](https://leetcode.com/submissions/detail/1148564441/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let map = arr.into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<i32, u16>, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
        let set = map.values().fold(HashSet::new(), |mut acc, y| {
            acc.insert(*y);
            acc
        });
        map.len() == set.len()
    }
}
```
