# 120. Triangle
- [Submission](https://leetcode.com/submissions/detail/1252907179/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in (0..triangle.len()-1).rev() {
            for j in 0..triangle[i].len() {
                triangle[i][j] += triangle[i+1][j].min(triangle[i+1][j+1]);
            }
        }
        triangle[0][0]
    }
}
```
