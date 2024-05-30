# 3075. Maximize Happiness of Selected Children
- [Submission](https://leetcode.com/submissions/detail/1253606450/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 20 ms | 5.2 MB |
```
impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        let n = happiness.len();
        happiness.sort_unstable_by_key(|x| -x);
        let mut happy = happiness[0] as i64;
        for i in 1..n {
            if k <= i as i32 || happiness[i] <= i as i32  { break; }
            happy += happiness[i] as i64 - i as i64;
        }
        happy
    }
}
```
