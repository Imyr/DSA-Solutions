# 57. Insert Interval
- [Submission](https://leetcode.com/submissions/detail/1264649778/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.6 MB |
```
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let idx = intervals.partition_point(|x| x[0] < new_interval[0]);
        if idx == intervals.len() { intervals.push(new_interval); }
        else { intervals.insert(idx, new_interval); }
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
