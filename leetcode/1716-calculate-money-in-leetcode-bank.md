# 1716. Calculate Money in Leetcode Bank
- [Submission](https://leetcode.com/submissions/detail/1113654277/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        28 * (n / 7) + 7 * (0..=(n / 7 - 1)).sum::<i32>() + (1..=(n % 7)).sum::<i32>() + (n / 7) * (n % 7)
    }
}
```
