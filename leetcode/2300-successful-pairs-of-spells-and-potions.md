# 2300. Successful Pairs of Spells and Potions
- [Submission](https://leetcode.com/submissions/detail/1136998540/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 38 ms | 4 MB |
```
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells.iter().map(|&s| {
            (potions.len() - potions.partition_point(|&p| (s as i64*p as i64) < success)) as i32
        }).collect()
    }
}
```
