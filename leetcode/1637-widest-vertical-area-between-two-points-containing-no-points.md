# 1637. Widest Vertical Area Between Two Points Containing No Points
- [Submission](https://leetcode.com/submissions/detail/1125061911/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 21 ms | 9.8 MB |
```
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut p = points.iter().map(|x| x[0]).collect::<Vec<i32>>();
        p.sort_unstable();
        p.windows(2).map(|x| x[1]-x[0]).max().unwrap()
    }
}
```
