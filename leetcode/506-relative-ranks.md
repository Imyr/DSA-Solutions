# 506. Relative Ranks
- [Submission](https://leetcode.com/submissions/detail/1252491453/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.6 MB |
```
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut pairs = score.into_iter().enumerate().collect::<Vec<_>>();
        pairs.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
        let mut string_pairs = pairs.into_iter().enumerate().map(|(i, (o, x))|
            (o, match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            })
        ).collect::<Vec<_>>();
        string_pairs.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        string_pairs.into_iter().map(|(_, x)| x).collect()
    }
}
```
