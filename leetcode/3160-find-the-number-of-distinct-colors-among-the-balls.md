# 3160. Find the Number of Distinct Colors Among the Balls
- [Submission](https://leetcode.com/submissions/detail/1267869567/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 60 ms | 11.7 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = HashMap::new();
        let mut map = HashMap::new();
        let mut result = vec![];
        for x in queries.into_iter() {
            if res.get(&x[0]).is_none() {
                res.entry(x[0]).or_insert(x[1]);
                // res[x[0] as usize] = x[1];
                *map.entry(x[1]).or_insert(0) += 1;
            } else if *res.get(&x[0]).unwrap() != x[1] {
                if map[res.get(&x[0]).unwrap()] == 1 { map.remove(res.get(&x[0]).unwrap()); }
                else { *map.get_mut(res.get(&x[0]).unwrap()).unwrap() -= 1; }
                *map.entry(x[1]).or_insert(0) += 1;
                res.insert(x[0], x[1]);
            }
            result.push(map.len() as i32);
        }
        result
    }
}
```
