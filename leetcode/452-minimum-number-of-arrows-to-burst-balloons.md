# 452. Minimum Number of Arrows to Burst Balloons
- [Submission](https://leetcode.com/submissions/detail/1265683816/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 37 ms | 9.4 MB |
```
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable(); let first = (points[0][0], points[0][1], 1);
        points.into_iter().skip(1).fold(first, |mut x, mut y| {
            if y[0] <= x.1 && y[0] >= x.0 { (y[0], y[1].min(x.1), x.2) } 
            else { (y[0], y[1], x.2+1) }
        }).2
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1265662946/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 44 ms | 9.3 MB |
```
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        let mut points = points.into_iter();
        let first = points.next().unwrap();
        points.fold(vec![first], |mut acc, mut x| {
            let n = acc.len()-1;
            if x[0] <= acc[n][1] && x[0] >= acc[n][0] {
                acc[n][0] = x[0];
                acc[n][1] = x[1].min(acc[n][1]);
            } else { acc.push(x) }
            acc
        }).len() as i32
    }
}
```
