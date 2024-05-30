# 1208. Get Equal Substrings Within Budget
- [Submission](https://leetcode.com/submissions/detail/1270349695/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.6 MB |
```
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let n = s.len();
        let prefix = (0..n).fold(vec![0], |mut acc, i| { acc.push(s[i].abs_diff(t[i]) as i32 + *acc.last().unwrap()); acc } );
        let mut max = 0;
        let (mut i, mut j) = (0, 0);
        while j <= n {
            if prefix[j]-prefix[i] <= max_cost { max = max.max(j-i); }
            else { i += 1; }
            j += 1;
        }
        max as _
    }
}
```
