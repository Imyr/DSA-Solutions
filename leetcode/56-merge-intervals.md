# 56. Merge Intervals
- [Submission](https://leetcode.com/submissions/detail/1264301362/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.9 MB |
```
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|x| (x[0], -x[1]));
        let mut intervals = intervals.into_iter();
        let first = intervals.next().unwrap();
        intervals.fold(vec![first], |mut acc, mut x| {
            let n = acc.len()-1;
            if acc[n][1] >= x[0] {
                acc[n][1] = x[1].max(acc[n][1]);
            } else { acc.push(x) }
            acc
        })
    }
}
```
