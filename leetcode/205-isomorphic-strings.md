# 205. Isomorphic Strings
- [Submission](https://leetcode.com/submissions/detail/1221182134/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.9 MB |
```
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut a = s.as_bytes().into_iter().enumerate()
                .fold(vec![vec![]; 128], |mut count, (idx, x)| {
                    count[*x as usize].push(idx);
                    count
                });
        let mut b = t.as_bytes().into_iter().enumerate()
                .fold(vec![vec![]; 128], |mut count, (idx, x)| {
                    count[*x as usize].push(idx);
                    count
                });
        // println!("{:?}\n{:?}", a, b);
        a.sort_unstable();
        b.sort_unstable();
        // println!("{:?}\n{:?}", a, b);
        a == b
    }
}
```
