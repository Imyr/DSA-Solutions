# 433. Minimum Genetic Mutation
- [Submission](https://leetcode.com/submissions/detail/1255559587/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
use std::collections::VecDeque;
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let s = start_gene.chars().collect::<Vec<_>>();
        let e = end_gene.chars().collect::<Vec<_>>();
        let mut bank = bank.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        bank.push(s.clone());
        bank.sort_unstable();
        if !bank.binary_search(&e).is_ok() { return -1; }
        let mut queue = (0..8).into_iter().map(|i| (0, i, e.clone())).collect::<VecDeque<_>>();

        while let Some((depth, i, g)) = queue.pop_front() {
            if g == s { return depth; }
            if g[i] != 'A' {
                let mut ng = g.clone();
                ng[i] = 'A';
                if bank.binary_search(&ng).is_ok() {
                    for j in 0..8 { if i != j { queue.push_back((depth+1, j, ng.clone())); } }
                }
            }
            if g[i] != 'G' {
                let mut ng = g.clone();
                ng[i] = 'G';
                if bank.binary_search(&ng).is_ok() {
                    for j in 0..8 { if i != j { queue.push_back((depth+1, j, ng.clone())); } }
                }
            }
            if g[i] != 'C' {
                let mut ng = g.clone();
                ng[i] = 'C';
                if bank.binary_search(&ng).is_ok() {
                    for j in 0..8 { if i != j { queue.push_back((depth+1, j, ng.clone())); } }
                }
            }
            if g[i] != 'T' {
                let mut ng = g.clone();
                ng[i] = 'T';
                if bank.binary_search(&ng).is_ok() {
                    for j in 0..8 { if i != j { queue.push_back((depth+1, j, ng.clone())); } }
                }
            }
        }
        -1
    }
}
```
