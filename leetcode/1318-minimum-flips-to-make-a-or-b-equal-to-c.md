# 1318. Minimum Flips to Make a OR b Equal to c
- [Submission](https://leetcode.com/submissions/detail/1136965008/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut flips = 0;
        for i in 0..32 {
            if a&1<<i | b&1<<i != c&1<<i {
                if c&1<<i == 0 {
                    if a&1<<i & b&1<<i == 1<<i {
                        flips += 2;
                    } else {
                        flips += 1;
                    }
                } else {
                    flips += 1;
                }
            }
        }
        flips
    }
}
```
