# 990. Satisfiability of Equality Equations
- [Submission](https://leetcode.com/submissions/detail/1269796844/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let (equal, unequal) = equations.into_iter().fold((HashMap::new(), HashMap::new()), |(mut e, mut ne), x| {
            let x = x.chars().collect::<Vec<char>>();
            match x[1] {
                '=' => { 
                    e.entry(x[0]).or_insert(HashSet::new()).insert(x[3]); 
                    e.entry(x[3]).or_insert(HashSet::new()).insert(x[0]); 
                    },
                '!' => {
                    ne.entry(x[0]).or_insert(HashSet::new()).insert(x[3]);
                    ne.entry(x[3]).or_insert(HashSet::new()).insert(x[0]);
                    },
                _ => unreachable!(),
            }
            (e, ne)
        });
        for c in 'a'..='z' {
            match (equal.get(&c), unequal.get(&c)) {
                (Some(a), Some(b)) => { 
                    if !a.is_disjoint(&b) { return false; }
                    let mut visited = HashSet::new();
                    visited.insert(&c);
                    let mut stack = vec![];
                    for x in a.iter() { stack.push(x); }
                    while let Some(c1) = stack.pop() {
                        if let Some(a1) = equal.get(&c1) {
                            if !a1.is_disjoint(&b) { return false; }
                            visited.insert(c1);
                            for c2 in a1.iter() { if !visited.contains(&c2) { stack.push(c2); } }
                        }
                    }
                },
                (None, Some(b)) => { if b.contains(&c) { return false; } }
                _ => continue,
            }
        }
        true
    }
}
```
