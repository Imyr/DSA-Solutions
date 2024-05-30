# 2225. Find Players With Zero or One Losses
- [Submission](https://leetcode.com/submissions/detail/1146640411/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 75 ms | 9.8 MB |
```
use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        matches.into_iter()
        .fold(HashMap::new(),  
            |mut acc, x| {
                acc.entry(x[0]).or_insert(0);
                *acc.entry(x[1]).or_insert(0) += 1;
                acc
            }
        ).into_iter()
        .filter(|y| y.1 <= 1)
        .fold(vec![vec![], vec![]], 
            |mut acc, z| {
                if z.1 == 0 { acc[0].push(z.0) }
                else if z.1 == 1 { acc[1].push(z.0) }
                acc
            }
        ).into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }).collect()
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1146626059/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 291 ms | 9.9 MB |
```
use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let matches = matches.into_iter().fold(HashMap::new(),  
            |mut acc, x| {
                acc.entry(x[0]).or_insert(0);
                *acc.entry(x[1]).or_insert(0) += 1;
                acc
            }
        );

        let mut losers = vec![vec![], vec![]];
        for m in matches.into_iter() {
            if m.1 == 0 { 
                let pos = losers[0].binary_search(&m.0).unwrap_err();
                losers[0].insert(pos, m.0); 
            } 
            else if m.1 == 1 { 
                let pos = losers[1].binary_search(&m.0).unwrap_err();
                losers[1].insert(pos, m.0); 
            }
        };
        losers
    }
}
```
