# 49. Group Anagrams
- [Submission](https://leetcode.com/submissions/detail/1261706544/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 6 MB |
```
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let n = strs.len();
        let map = strs.iter().map(|x| 
            x.chars().fold(vec![0; 26], |mut acc, c| {
                acc[c as usize - 'a' as usize] += 1;
                acc
        })).collect::<Vec<_>>();
        let mut hashmap = std::collections::HashMap::new();
        for i in 0..n { hashmap.entry(&map[i]).or_insert(vec![]).push(strs[i].clone()); }
        hashmap.into_iter().map(|(_, x)| x).collect()
    }
}
```
