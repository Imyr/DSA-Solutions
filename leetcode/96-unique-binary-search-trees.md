# 96. Unique Binary Search Trees
- [Submission](https://leetcode.com/submissions/detail/1270614700/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
use std::collections::HashMap;
impl Solution {
    fn recurse(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 || n == 1 { 1 }
        else if let Some(x) = cache.get(&n) { *x } 
        else {
            let num = (0..n).fold(0, |mut acc, i| { acc += Self::recurse(i, cache)*Self::recurse(n-i-1, cache); acc } );
            cache.insert(n, num);
            num
        }
    }
    pub fn num_trees(n: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::recurse(n, &mut cache)
    }
}
```
