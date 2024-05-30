# 552. Student Attendance Record II
- [Submission](https://leetcode.com/submissions/detail/1268291723/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1596 ms | 46 MB |
```
use std::collections::HashMap;
impl Solution {
    fn check(a: i32, l: i32, n: i32, cache: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
        if n == 1 { a + l.min(1) + 1 }
        else if let Some(val) = cache.get(&(a, l, n)) { *val }
        else {
            let p = Self::check(a, 2, n-1, cache) % 1000000007;
            let ab = (if a == 1 { Self::check(0, 2, n-1, cache) } else { 0 }) % 1000000007;
            let la = (if l >= 1 { Self::check(a, l-1, n-1, cache) } else { 0 }) % 1000000007;
            let max = ( (ab + la) % 1000000007 + p ) % 1000000007;
            cache.insert((a, l, n), max);
            max 
        }
    }
    pub fn check_record(n: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::check(1, 2, n, &mut cache)
    }
}
```
