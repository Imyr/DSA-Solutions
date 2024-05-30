# 1347. Minimum Number of Steps to Make Two Strings Anagram
- [Submission](https://leetcode.com/submissions/detail/1144810348/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.4 MB |
```
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        s.as_bytes()
        .into_iter()
        .fold(
            vec![0; 26], |mut count, x| {
                count[*x as usize - 97] += 1;
                count
        }).into_iter().zip(
            t.as_bytes()
            .into_iter()
            .fold(
                vec![0; 26], |mut count, x| {
                    count[*x as usize - 97] += 1;
                    count
            }).into_iter()
        ).map(|(a, b)| (a-b).max(0)).sum()
    }
}
```
